use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::sync::OnceLock;

use serde::Deserialize;

const CORPUS_ROOT: &str = "tests/corpus/csstree";
const REPORT_PATH: &str = "expectations/generation-reports/all.json";
const SOURCE_SIDECAR_PATH: &str = "source/.surgeist-source.json";
const EXPECTED_MANIFEST_DIGEST: &str =
    "dc61d7c2cd387418e1a9b403e2aa71266abfdf291f95c7598688d48452cec96e";
const EXPECTED_SOURCE_REPOSITORY: &str = "https://github.com/csstree/csstree.git";
const EXPECTED_SOURCE_REVISION: &str = "88e3d965c0b1628642a30a841745b410d6835052";
const EXPECTED_IMPORT_PROVENANCE: &str =
    "f4f2957f38bf42d052a2593b94f853b6f17255eadd2ac26928c8bebe6bb3423c";
const EXPECTED_GENERATOR: &str = "surgeist-css-generate";
const EXPECTED_ARTIFACTS: usize = 74;
const EXPECTED_CASES: usize = 935;
const EXPECTED_PARSED: usize = 721;
const EXPECTED_REJECTED: usize = 214;
const EXPECTED_CONTEXT_COUNTS: [(Context, usize); 11] = [
    (Context::Atrule, 130),
    (Context::AtrulePrelude, 2),
    (Context::Block, 29),
    (Context::Declaration, 77),
    (Context::DeclarationList, 19),
    (Context::MediaQuery, 49),
    (Context::Rule, 33),
    (Context::Selector, 317),
    (Context::SelectorList, 10),
    (Context::Stylesheet, 76),
    (Context::Value, 193),
];

static CORPUS: OnceLock<Result<Corpus, String>> = OnceLock::new();

pub(crate) fn load_csstree_corpus() -> Result<&'static Corpus, &'static str> {
    match CORPUS.get_or_init(|| {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(CORPUS_ROOT);
        read_artifact_set(&root).and_then(validate_artifact_set)
    }) {
        Ok(corpus) => Ok(corpus),
        Err(error) => Err(error.as_str()),
    }
}

#[derive(Debug)]
pub(crate) struct Corpus {
    artifact_count: usize,
    cases: Vec<ValidatedCase>,
    parsed_count: usize,
    rejected_count: usize,
    context_counts: [usize; 11],
}

impl Corpus {
    pub(crate) fn artifact_count(&self) -> usize {
        self.artifact_count
    }

    pub(crate) fn case_count(&self) -> usize {
        self.cases.len()
    }

    pub(crate) fn parsed_count(&self) -> usize {
        self.parsed_count
    }

    pub(crate) fn rejected_count(&self) -> usize {
        self.rejected_count
    }

    pub(crate) fn context_counts(&self) -> [(&'static str, usize); 11] {
        std::array::from_fn(|index| {
            (
                EXPECTED_CONTEXT_COUNTS[index].0.as_str(),
                self.context_counts[index],
            )
        })
    }
}

#[derive(Debug)]
struct ValidatedCase {
    _id: String,
    _context: Context,
    _label: Option<String>,
    _input: String,
    _upstream_outcome: UpstreamOutcome,
    _canonical_css: Option<String>,
    _options: Option<CaseOptions>,
}

#[derive(Debug)]
struct ArtifactSet {
    report: Vec<u8>,
    expectations: BTreeMap<String, Vec<u8>>,
    sources: BTreeMap<String, Vec<u8>>,
}

fn read_artifact_set(root: &Path) -> Result<ArtifactSet, String> {
    let report_path = root.join(REPORT_PATH);

    Ok(ArtifactSet {
        report: read_file(&report_path)?,
        expectations: read_json_artifacts(root, "expectations", Some(REPORT_PATH))?,
        sources: read_json_artifacts(root, "source", Some(SOURCE_SIDECAR_PATH))?,
    })
}

fn read_json_artifacts(
    root: &Path,
    relative_root: &str,
    excluded_path: Option<&str>,
) -> Result<BTreeMap<String, Vec<u8>>, String> {
    let mut pending = vec![root.join(relative_root)];
    let mut artifacts = BTreeMap::new();

    while let Some(directory) = pending.pop() {
        let entries = fs::read_dir(&directory)
            .map_err(|error| format!("failed to read {}: {error}", directory.display()))?;
        for entry in entries {
            let entry = entry.map_err(|error| {
                format!("failed to read entry in {}: {error}", directory.display())
            })?;
            let file_type = entry.file_type().map_err(|error| {
                format!("failed to inspect {}: {error}", entry.path().display())
            })?;
            let path = entry.path();
            if file_type.is_dir() {
                pending.push(path);
                continue;
            }
            if !file_type.is_file()
                || path.extension().and_then(|value| value.to_str()) != Some("json")
            {
                continue;
            }

            let relative = relative_path(root, &path)?;
            if excluded_path == Some(relative.as_str()) {
                continue;
            }
            let bytes = read_file(&path)?;
            if artifacts.insert(relative.clone(), bytes).is_some() {
                return Err(format!("duplicate artifact path {relative}"));
            }
        }
    }

    Ok(artifacts)
}

fn read_file(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn relative_path(root: &Path, path: &Path) -> Result<String, String> {
    let relative = path.strip_prefix(root).map_err(|error| {
        format!(
            "{} is not beneath corpus root {}: {error}",
            path.display(),
            root.display()
        )
    })?;
    let relative = relative
        .to_str()
        .ok_or_else(|| format!("artifact path is not UTF-8: {}", path.display()))?
        .replace(std::path::MAIN_SEPARATOR, "/");
    if !is_canonical_relative_path(&relative) {
        return Err(format!("artifact path is not canonical: {relative}"));
    }
    Ok(relative)
}

fn validate_artifact_set(artifacts: ArtifactSet) -> Result<Corpus, String> {
    let report: RawReport = serde_json::from_slice(&artifacts.report)
        .map_err(|error| format!("failed to deserialize corpus report: {error}"))?;
    validate_report_header(&report)?;

    if report.artifacts.len() != EXPECTED_ARTIFACTS {
        return Err(format!(
            "report artifact census mismatch: expected {EXPECTED_ARTIFACTS}, found {}",
            report.artifacts.len()
        ));
    }

    let report_expectations = report
        .artifacts
        .iter()
        .map(|artifact| artifact.output_path.clone())
        .collect::<BTreeSet<_>>();
    let report_sources = report
        .artifacts
        .iter()
        .map(|artifact| artifact.provenance.source_path.clone())
        .collect::<BTreeSet<_>>();
    let disk_expectations = artifacts.expectations.keys().cloned().collect();
    let disk_sources = artifacts.sources.keys().cloned().collect();
    if report_expectations != disk_expectations {
        return Err(set_mismatch(
            "expectation artifact inventory",
            &report_expectations,
            &disk_expectations,
        ));
    }
    if report_sources != disk_sources {
        return Err(set_mismatch(
            "source artifact inventory",
            &report_sources,
            &disk_sources,
        ));
    }

    let mut cases = Vec::with_capacity(EXPECTED_CASES);
    let mut seen_ids = BTreeSet::new();
    let mut previous_id: Option<String> = None;
    let mut previous_output_path: Option<&str> = None;
    let mut parsed_count = 0;
    let mut rejected_count = 0;
    let mut context_counts = [0; 11];

    for artifact in &report.artifacts {
        if previous_output_path.is_some_and(|previous| previous >= artifact.output_path.as_str()) {
            return Err(format!(
                "report artifacts are not in canonical order at {}",
                artifact.output_path
            ));
        }
        previous_output_path = Some(&artifact.output_path);

        validate_report_artifact(artifact)?;
        let expectation_bytes = artifacts
            .expectations
            .get(&artifact.output_path)
            .ok_or_else(|| format!("missing expectation artifact {}", artifact.output_path))?;
        let source_bytes = artifacts
            .sources
            .get(&artifact.provenance.source_path)
            .ok_or_else(|| {
                format!(
                    "missing source artifact {}",
                    artifact.provenance.source_path
                )
            })?;
        let expectation: RawExpectation =
            serde_json::from_slice(expectation_bytes).map_err(|error| {
                format!(
                    "failed to deserialize expectation {}: {error}",
                    artifact.output_path
                )
            })?;
        validate_expectation_header(&expectation, artifact)?;

        if expectation.cases.len() != artifact.case_count {
            return Err(format!(
                "case census mismatch for {}: report {}, expectation {}",
                artifact.output_path,
                artifact.case_count,
                expectation.cases.len()
            ));
        }

        let expected_output_digest = sha256_hex(expectation_bytes);
        if artifact.output_digest != expected_output_digest {
            return Err(format!(
                "output digest mismatch for {}: report {}, actual {expected_output_digest}",
                artifact.output_path, artifact.output_digest
            ));
        }
        let expected_source_digest = sha256_hex(source_bytes);
        if artifact.provenance.source_digest != expected_source_digest {
            return Err(format!(
                "source digest mismatch for {}: report {}, actual {expected_source_digest}",
                artifact.provenance.source_path, artifact.provenance.source_digest
            ));
        }
        if expectation.source_sha256 != expected_source_digest {
            return Err(format!(
                "expectation source digest mismatch for {}: expectation {}, actual {expected_source_digest}",
                artifact.output_path, expectation.source_sha256
            ));
        }

        let source_suffix = artifact
            .provenance
            .source_path
            .strip_prefix("source/")
            .ok_or_else(|| {
                format!(
                    "source path lacks canonical prefix: {}",
                    artifact.provenance.source_path
                )
            })?;
        let expected_id_prefix = format!("{source_suffix}#/");

        for raw_case in expectation.cases {
            if !raw_case.id.starts_with(&expected_id_prefix) {
                return Err(format!(
                    "case ID {} is not bound to source {}",
                    raw_case.id, artifact.provenance.source_path
                ));
            }
            if !seen_ids.insert(raw_case.id.clone()) {
                return Err(format!("duplicate case ID {}", raw_case.id));
            }
            if previous_id
                .as_deref()
                .is_some_and(|previous| previous >= raw_case.id.as_str())
            {
                return Err(format!(
                    "case IDs are not in canonical order at {}",
                    raw_case.id
                ));
            }
            previous_id = Some(raw_case.id.clone());

            if raw_case.status != Disposition::Active {
                return Err(format!("case {} has non-active disposition", raw_case.id));
            }
            if raw_case.reason.is_some() {
                return Err(format!(
                    "active case {} carries a disposition reason",
                    raw_case.id
                ));
            }

            match raw_case.upstream_outcome {
                UpstreamOutcome::Parsed => parsed_count += 1,
                UpstreamOutcome::Rejected => rejected_count += 1,
            }
            context_counts[raw_case.context.index()] += 1;
            cases.push(ValidatedCase {
                _id: raw_case.id,
                _context: raw_case.context,
                _label: raw_case.label,
                _input: raw_case.input,
                _upstream_outcome: raw_case.upstream_outcome,
                _canonical_css: raw_case.canonical_css,
                _options: raw_case.options,
            });
        }
    }

    validate_census(
        &report,
        &cases,
        parsed_count,
        rejected_count,
        context_counts,
    )?;

    Ok(Corpus {
        artifact_count: report.artifacts.len(),
        cases,
        parsed_count,
        rejected_count,
        context_counts,
    })
}

fn validate_report_header(report: &RawReport) -> Result<(), String> {
    if report.manifest_digest != EXPECTED_MANIFEST_DIGEST {
        return Err(format!(
            "report manifest digest mismatch: expected {EXPECTED_MANIFEST_DIGEST}, report {}",
            report.manifest_digest
        ));
    }
    if report.source_repository != EXPECTED_SOURCE_REPOSITORY {
        return Err(format!(
            "unexpected source repository {}",
            report.source_repository
        ));
    }
    if report.source_revision != EXPECTED_SOURCE_REVISION {
        return Err(format!(
            "unexpected source revision {}",
            report.source_revision
        ));
    }
    if report.counts
        != (ReportCounts {
            active: EXPECTED_CASES,
            expected_fail: 0,
            unsupported: 0,
            quarantined: 0,
            failed_to_generate: 0,
        })
    {
        return Err(format!(
            "unexpected report disposition counts: {:?}",
            report.counts
        ));
    }
    Ok(())
}

fn validate_report_artifact(artifact: &ReportArtifact) -> Result<(), String> {
    if !is_canonical_relative_path(&artifact.output_path)
        || !artifact.output_path.starts_with("expectations/")
        || !artifact.output_path.ends_with(".json")
    {
        return Err(format!(
            "non-canonical expectation path {}",
            artifact.output_path
        ));
    }
    if !is_canonical_relative_path(&artifact.provenance.source_path)
        || !artifact.provenance.source_path.starts_with("source/")
        || !artifact.provenance.source_path.ends_with(".json")
    {
        return Err(format!(
            "non-canonical source path {}",
            artifact.provenance.source_path
        ));
    }
    let output_suffix = artifact
        .output_path
        .strip_prefix("expectations/")
        .expect("validated expectation prefix");
    let source_suffix = artifact
        .provenance
        .source_path
        .strip_prefix("source/")
        .expect("validated source prefix");
    if output_suffix != source_suffix {
        return Err(format!(
            "report path pair does not share a suffix: {} and {}",
            artifact.output_path, artifact.provenance.source_path
        ));
    }
    if artifact.provenance.generator != EXPECTED_GENERATOR {
        return Err(format!(
            "unexpected generator {} for {}",
            artifact.provenance.generator, artifact.output_path
        ));
    }
    if artifact.provenance.schema_version != 1 {
        return Err(format!(
            "unsupported report schema version {} for {}",
            artifact.provenance.schema_version, artifact.output_path
        ));
    }
    if artifact.provenance.domain_provenance.csstree_import != EXPECTED_IMPORT_PROVENANCE {
        return Err(format!(
            "unexpected import provenance for {}",
            artifact.output_path
        ));
    }
    validate_digest("report output", &artifact.output_digest)?;
    validate_digest("report source", &artifact.provenance.source_digest)?;
    Ok(())
}

fn validate_expectation_header(
    expectation: &RawExpectation,
    artifact: &ReportArtifact,
) -> Result<(), String> {
    if expectation.schema_version != 1 {
        return Err(format!(
            "unsupported expectation schema version {} for {}",
            expectation.schema_version, artifact.output_path
        ));
    }
    if expectation.generator != EXPECTED_GENERATOR {
        return Err(format!(
            "unexpected expectation generator {} for {}",
            expectation.generator, artifact.output_path
        ));
    }
    if expectation.source != artifact.provenance.source_path {
        return Err(format!(
            "expectation source mismatch for {}: expectation {}, report {}",
            artifact.output_path, expectation.source, artifact.provenance.source_path
        ));
    }
    if expectation.source_sha256 != artifact.provenance.source_digest {
        return Err(format!(
            "expectation/report source digest mismatch for {}",
            artifact.output_path
        ));
    }
    if expectation.source_revision != EXPECTED_SOURCE_REVISION {
        return Err(format!(
            "unexpected expectation source revision {} for {}",
            expectation.source_revision, artifact.output_path
        ));
    }
    if expectation.import_provenance_sha256 != EXPECTED_IMPORT_PROVENANCE
        || expectation.import_provenance_sha256
            != artifact.provenance.domain_provenance.csstree_import
    {
        return Err(format!(
            "expectation import provenance mismatch for {}",
            artifact.output_path
        ));
    }
    validate_digest("expectation source", &expectation.source_sha256)?;
    Ok(())
}

fn validate_census(
    report: &RawReport,
    cases: &[ValidatedCase],
    parsed_count: usize,
    rejected_count: usize,
    context_counts: [usize; 11],
) -> Result<(), String> {
    if cases.len() != EXPECTED_CASES {
        return Err(format!(
            "case census mismatch: expected {EXPECTED_CASES}, found {}",
            cases.len()
        ));
    }
    if parsed_count != EXPECTED_PARSED || rejected_count != EXPECTED_REJECTED {
        return Err(format!(
            "outcome census mismatch: expected {EXPECTED_PARSED}/{EXPECTED_REJECTED} parsed/rejected, found {parsed_count}/{rejected_count}"
        ));
    }
    if context_counts != EXPECTED_CONTEXT_COUNTS.map(|(_, expected_count)| expected_count) {
        return Err(format!("context census mismatch: {context_counts:?}"));
    }
    if report.counts.active != cases.len() {
        return Err(format!(
            "report active census mismatch: report {}, loaded {}",
            report.counts.active,
            cases.len()
        ));
    }
    Ok(())
}

fn set_mismatch(label: &str, expected: &BTreeSet<String>, actual: &BTreeSet<String>) -> String {
    let missing = expected.difference(actual).cloned().collect::<Vec<_>>();
    let extra = actual.difference(expected).cloned().collect::<Vec<_>>();
    format!("{label} mismatch: missing {missing:?}, extra {extra:?}")
}

fn is_canonical_relative_path(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains('\\')
        && path
            .split('/')
            .all(|component| !component.is_empty() && component != "." && component != "..")
}

fn validate_digest(label: &str, digest: &str) -> Result<(), String> {
    if digest.len() == 64
        && digest
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        Ok(())
    } else {
        Err(format!(
            "{label} digest is not canonical lowercase SHA-256: {digest}"
        ))
    }
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawReport {
    manifest_digest: String,
    source_repository: String,
    source_revision: String,
    counts: ReportCounts,
    artifacts: Vec<ReportArtifact>,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct ReportCounts {
    active: usize,
    expected_fail: usize,
    unsupported: usize,
    quarantined: usize,
    failed_to_generate: usize,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReportArtifact {
    provenance: ReportProvenance,
    output_path: String,
    output_digest: String,
    case_count: usize,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReportProvenance {
    source_path: String,
    source_digest: String,
    generator: String,
    schema_version: u64,
    domain_provenance: DomainProvenance,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DomainProvenance {
    #[serde(rename = "csstree-import")]
    csstree_import: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawExpectation {
    schema_version: u64,
    generator: String,
    source: String,
    source_sha256: String,
    source_revision: String,
    import_provenance_sha256: String,
    cases: Vec<RawCase>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawCase {
    id: String,
    context: Context,
    #[serde(default)]
    label: Option<String>,
    input: String,
    upstream_outcome: UpstreamOutcome,
    #[serde(default)]
    canonical_css: Option<String>,
    #[serde(default)]
    options: Option<CaseOptions>,
    status: Disposition,
    #[serde(default, deserialize_with = "deserialize_present_optional_string")]
    reason: Option<Option<String>>,
}

fn deserialize_present_optional_string<'de, D>(
    deserializer: D,
) -> Result<Option<Option<String>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    Option::<String>::deserialize(deserializer).map(Some)
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CaseOptions {
    #[serde(default, rename = "atrule")]
    _atrule: Option<String>,
    #[serde(default, rename = "parseAtrulePrelude")]
    _parse_atrule_prelude: Option<bool>,
    #[serde(default, rename = "parseCustomProperty")]
    _parse_custom_property: Option<bool>,
    #[serde(default, rename = "parseRulePrelude")]
    _parse_rule_prelude: Option<bool>,
    #[serde(default, rename = "parseValue")]
    _parse_value: Option<bool>,
    #[serde(default, rename = "property")]
    _property: Option<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
enum Context {
    #[serde(rename = "atrule")]
    Atrule,
    #[serde(rename = "atrulePrelude")]
    AtrulePrelude,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "declaration")]
    Declaration,
    #[serde(rename = "declarationList")]
    DeclarationList,
    #[serde(rename = "mediaQuery")]
    MediaQuery,
    #[serde(rename = "rule")]
    Rule,
    #[serde(rename = "selector")]
    Selector,
    #[serde(rename = "selectorList")]
    SelectorList,
    #[serde(rename = "stylesheet")]
    Stylesheet,
    #[serde(rename = "value")]
    Value,
}

impl Context {
    const fn index(self) -> usize {
        match self {
            Self::Atrule => 0,
            Self::AtrulePrelude => 1,
            Self::Block => 2,
            Self::Declaration => 3,
            Self::DeclarationList => 4,
            Self::MediaQuery => 5,
            Self::Rule => 6,
            Self::Selector => 7,
            Self::SelectorList => 8,
            Self::Stylesheet => 9,
            Self::Value => 10,
        }
    }

    const fn as_str(self) -> &'static str {
        match self {
            Self::Atrule => "atrule",
            Self::AtrulePrelude => "atrulePrelude",
            Self::Block => "block",
            Self::Declaration => "declaration",
            Self::DeclarationList => "declarationList",
            Self::MediaQuery => "mediaQuery",
            Self::Rule => "rule",
            Self::Selector => "selector",
            Self::SelectorList => "selectorList",
            Self::Stylesheet => "stylesheet",
            Self::Value => "value",
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
enum UpstreamOutcome {
    Parsed,
    Rejected,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum Disposition {
    Active,
    ExpectedFail,
    Unsupported,
    Quarantined,
}

fn sha256_hex(input: &[u8]) -> String {
    const INITIAL: [u32; 8] = [
        0x6a09_e667,
        0xbb67_ae85,
        0x3c6e_f372,
        0xa54f_f53a,
        0x510e_527f,
        0x9b05_688c,
        0x1f83_d9ab,
        0x5be0_cd19,
    ];
    const ROUND: [u32; 64] = [
        0x428a_2f98,
        0x7137_4491,
        0xb5c0_fbcf,
        0xe9b5_dba5,
        0x3956_c25b,
        0x59f1_11f1,
        0x923f_82a4,
        0xab1c_5ed5,
        0xd807_aa98,
        0x1283_5b01,
        0x2431_85be,
        0x550c_7dc3,
        0x72be_5d74,
        0x80de_b1fe,
        0x9bdc_06a7,
        0xc19b_f174,
        0xe49b_69c1,
        0xefbe_4786,
        0x0fc1_9dc6,
        0x240c_a1cc,
        0x2de9_2c6f,
        0x4a74_84aa,
        0x5cb0_a9dc,
        0x76f9_88da,
        0x983e_5152,
        0xa831_c66d,
        0xb003_27c8,
        0xbf59_7fc7,
        0xc6e0_0bf3,
        0xd5a7_9147,
        0x06ca_6351,
        0x1429_2967,
        0x27b7_0a85,
        0x2e1b_2138,
        0x4d2c_6dfc,
        0x5338_0d13,
        0x650a_7354,
        0x766a_0abb,
        0x81c2_c92e,
        0x9272_2c85,
        0xa2bf_e8a1,
        0xa81a_664b,
        0xc24b_8b70,
        0xc76c_51a3,
        0xd192_e819,
        0xd699_0624,
        0xf40e_3585,
        0x106a_a070,
        0x19a4_c116,
        0x1e37_6c08,
        0x2748_774c,
        0x34b0_bcb5,
        0x391c_0cb3,
        0x4ed8_aa4a,
        0x5b9c_ca4f,
        0x682e_6ff3,
        0x748f_82ee,
        0x78a5_636f,
        0x84c8_7814,
        0x8cc7_0208,
        0x90be_fffa,
        0xa450_6ceb,
        0xbef9_a3f7,
        0xc671_78f2,
    ];

    let bit_len = (input.len() as u64).wrapping_mul(8);
    let mut padded = input.to_vec();
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bit_len.to_be_bytes());

    let mut state = INITIAL;
    for chunk in padded.chunks_exact(64) {
        let mut words = [0_u32; 64];
        for (index, word) in words.iter_mut().take(16).enumerate() {
            let start = index * 4;
            *word = u32::from_be_bytes([
                chunk[start],
                chunk[start + 1],
                chunk[start + 2],
                chunk[start + 3],
            ]);
        }
        for index in 16..64 {
            let s0 = words[index - 15].rotate_right(7)
                ^ words[index - 15].rotate_right(18)
                ^ (words[index - 15] >> 3);
            let s1 = words[index - 2].rotate_right(17)
                ^ words[index - 2].rotate_right(19)
                ^ (words[index - 2] >> 10);
            words[index] = words[index - 16]
                .wrapping_add(s0)
                .wrapping_add(words[index - 7])
                .wrapping_add(s1);
        }

        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;
        for index in 0..64 {
            let sum1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choose = (e & f) ^ (!e & g);
            let temp1 = h
                .wrapping_add(sum1)
                .wrapping_add(choose)
                .wrapping_add(ROUND[index])
                .wrapping_add(words[index]);
            let sum0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = sum0.wrapping_add(majority);

            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        for (slot, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *slot = slot.wrapping_add(value);
        }
    }

    let mut digest = String::with_capacity(64);
    for word in state {
        use std::fmt::Write as _;
        write!(&mut digest, "{word:08x}").expect("writing to String cannot fail");
    }
    digest
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    fn committed_artifacts() -> ArtifactSet {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(CORPUS_ROOT);
        read_artifact_set(&root).expect("committed artifacts should be readable")
    }

    fn mutate_first_expectation(artifacts: &mut ArtifactSet, edit: impl FnOnce(&mut Value)) {
        let path = artifacts
            .expectations
            .keys()
            .next()
            .expect("expectation inventory")
            .clone();
        let mut document: Value =
            serde_json::from_slice(&artifacts.expectations[&path]).expect("expectation JSON");
        edit(&mut document);
        let bytes = serde_json::to_vec(&document).expect("serialize mutated expectation");
        let digest = sha256_hex(&bytes);
        artifacts.expectations.insert(path.clone(), bytes);
        mutate_report(artifacts, |report| {
            let artifact = report["artifacts"]
                .as_array_mut()
                .expect("artifact array")
                .iter_mut()
                .find(|artifact| artifact["output_path"] == path)
                .expect("report artifact");
            artifact["output_digest"] = Value::from(digest);
        });
    }

    fn mutate_report(artifacts: &mut ArtifactSet, edit: impl FnOnce(&mut Value)) {
        let mut report: Value =
            serde_json::from_slice(&artifacts.report).expect("report JSON should deserialize");
        edit(&mut report);
        artifacts.report = serde_json::to_vec(&report).expect("serialize mutated report");
    }

    fn assert_rejected(artifacts: ArtifactSet, expected: &str) {
        let error = validate_artifact_set(artifacts).expect_err("artifact set should be rejected");
        assert!(
            error.contains(expected),
            "expected error containing {expected:?}, got {error:?}"
        );
    }

    #[test]
    fn sha256_matches_known_vector() {
        assert_eq!(
            sha256_hex(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
    }

    #[test]
    fn loader_rejects_malformed_schema() {
        let mut artifacts = committed_artifacts();
        mutate_first_expectation(&mut artifacts, |document| {
            document["schema_version"] = Value::from(2);
        });
        assert_rejected(artifacts, "expectation schema version");
    }

    #[test]
    fn loader_rejects_malformed_report_schema() {
        let mut artifacts = committed_artifacts();
        mutate_report(&mut artifacts, |report| {
            report["artifacts"][0]["provenance"]["schema_version"] = Value::from(2);
        });
        assert_rejected(artifacts, "report schema version");
    }

    #[test]
    fn loader_rejects_duplicate_ids() {
        let mut artifacts = committed_artifacts();
        mutate_first_expectation(&mut artifacts, |document| {
            let first_id = document["cases"][0]["id"].clone();
            document["cases"][1]["id"] = first_id;
        });
        assert_rejected(artifacts, "duplicate case ID");
    }

    #[test]
    fn loader_rejects_unknown_context() {
        let mut artifacts = committed_artifacts();
        mutate_first_expectation(&mut artifacts, |document| {
            document["cases"][0]["context"] = Value::from("unknown");
        });
        assert_rejected(artifacts, "failed to deserialize expectation");
    }

    #[test]
    fn loader_rejects_unknown_outcome() {
        let mut artifacts = committed_artifacts();
        mutate_first_expectation(&mut artifacts, |document| {
            document["cases"][0]["upstream_outcome"] = Value::from("unknown");
        });
        assert_rejected(artifacts, "failed to deserialize expectation");
    }

    #[test]
    fn loader_rejects_non_active_disposition() {
        let mut artifacts = committed_artifacts();
        mutate_first_expectation(&mut artifacts, |document| {
            document["cases"][0]["status"] = Value::from("expected-fail");
            document["cases"][0]["reason"] = Value::from("known mismatch");
        });
        assert_rejected(artifacts, "non-active disposition");
    }

    #[test]
    fn loader_rejects_reason_on_active_case() {
        let mut artifacts = committed_artifacts();
        mutate_first_expectation(&mut artifacts, |document| {
            document["cases"][0]["reason"] = Value::from("not neutral");
        });
        assert_rejected(artifacts, "carries a disposition reason");
    }

    #[test]
    fn loader_rejects_null_reason_on_active_case() {
        let mut artifacts = committed_artifacts();
        mutate_first_expectation(&mut artifacts, |document| {
            document["cases"][0]["reason"] = Value::Null;
        });
        assert_rejected(artifacts, "carries a disposition reason");
    }

    #[test]
    fn loader_rejects_unknown_case_option() {
        let mut artifacts = committed_artifacts();
        mutate_first_expectation(&mut artifacts, |document| {
            document["cases"][0]["options"]["unknown"] = Value::Bool(true);
        });
        assert_rejected(artifacts, "failed to deserialize expectation");
    }

    #[test]
    fn loader_rejects_stale_report_output_digest() {
        let mut artifacts = committed_artifacts();
        mutate_report(&mut artifacts, |report| {
            report["artifacts"][0]["output_digest"] = Value::from("0".repeat(64));
        });
        assert_rejected(artifacts, "output digest mismatch");
    }

    #[test]
    fn loader_rejects_stale_report_manifest_digest() {
        let mut artifacts = committed_artifacts();
        mutate_report(&mut artifacts, |report| {
            report["manifest_digest"] = Value::from("0".repeat(64));
        });
        assert_rejected(artifacts, "report manifest digest mismatch");
    }

    #[test]
    fn loader_rejects_stale_source_digest() {
        let mut artifacts = committed_artifacts();
        mutate_report(&mut artifacts, |report| {
            report["artifacts"][0]["provenance"]["source_digest"] = Value::from("0".repeat(64));
        });
        assert_rejected(artifacts, "expectation/report source digest mismatch");
    }

    #[test]
    fn loader_rejects_missing_expectation_artifact() {
        let mut artifacts = committed_artifacts();
        let path = artifacts
            .expectations
            .keys()
            .next()
            .expect("expectation inventory")
            .clone();
        artifacts.expectations.remove(&path);
        assert_rejected(artifacts, "expectation artifact inventory mismatch");
    }

    #[test]
    fn loader_rejects_extra_expectation_artifact() {
        let mut artifacts = committed_artifacts();
        artifacts
            .expectations
            .insert("expectations/extra.json".into(), b"{}".to_vec());
        assert_rejected(artifacts, "expectation artifact inventory mismatch");
    }

    #[test]
    fn loader_rejects_noncanonical_report_order() {
        let mut artifacts = committed_artifacts();
        mutate_report(&mut artifacts, |report| {
            report["artifacts"]
                .as_array_mut()
                .expect("artifact array")
                .swap(0, 1);
        });
        assert_rejected(artifacts, "report artifacts are not in canonical order");
    }
}
