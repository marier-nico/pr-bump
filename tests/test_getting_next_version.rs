use pr_bump_lib::{self, get_next_version, LocalGitHub, PrBumpConfig, PullRequest, Release};

mod utils;
use semver::Version;
use utils::ymd_midnight;

#[tokio::test]
async fn fix_pr_bumps_patch_number() {
    let mut gh = LocalGitHub::default();
    gh.add_release(Release::new("1.2.3".to_string(), ymd_midnight(2021, 1, 1)));
    gh.add_pull(PullRequest::new(
        vec!["fix".to_string()],
        Some(ymd_midnight(2021, 1, 2)),
    ));

    let next_version = get_next_version(gh, PrBumpConfig::default()).await.unwrap();

    assert_eq!(next_version, Version::new(1, 2, 4));
}

#[tokio::test]
async fn feature_pr_bumps_minor_number() {
    let mut gh = LocalGitHub::default();
    gh.add_release(Release::new("1.2.3".to_string(), ymd_midnight(2021, 1, 1)));
    gh.add_pull(PullRequest::new(
        vec!["feature".to_string()],
        Some(ymd_midnight(2021, 1, 2)),
    ));

    let next_version = get_next_version(gh, PrBumpConfig::default()).await.unwrap();

    assert_eq!(next_version, Version::new(1, 3, 0));
}

#[tokio::test]
async fn breaking_pr_bumps_major_number() {
    let mut gh = LocalGitHub::default();
    gh.add_release(Release::new("1.2.3".to_string(), ymd_midnight(2021, 1, 1)));
    gh.add_pull(PullRequest::new(
        vec!["breaking".to_string()],
        Some(ymd_midnight(2021, 1, 2)),
    ));

    let next_version = get_next_version(gh, PrBumpConfig::default()).await.unwrap();

    assert_eq!(next_version, Version::new(2, 0, 0));
}

#[tokio::test]
async fn no_new_prs_does_not_bump() {
    let mut gh = LocalGitHub::default();
    gh.add_release(Release::new("1.2.3".to_string(), ymd_midnight(2021, 1, 1)));

    let next_version = get_next_version(gh, PrBumpConfig::default()).await.unwrap();

    assert_eq!(next_version, Version::new(1, 2, 3));
}

#[tokio::test]
async fn prs_merged_before_latest_release_do_not_bump() {
    let mut gh = LocalGitHub::default();
    gh.add_release(Release::new("1.2.3".to_string(), ymd_midnight(2021, 1, 1)));
    gh.add_pull(PullRequest::new(
        vec!["breaking".to_string()],
        Some(ymd_midnight(2020, 1, 1)),
    ));

    let next_version = get_next_version(gh, PrBumpConfig::default()).await.unwrap();

    assert_eq!(next_version, Version::new(1, 2, 3));
}

#[tokio::test]
async fn prs_with_no_labels_do_not_bump() {
    let mut gh = LocalGitHub::default();
    gh.add_release(Release::new("1.2.3".to_string(), ymd_midnight(2021, 1, 1)));
    gh.add_pull(PullRequest::new(vec![], Some(ymd_midnight(2020, 1, 1))));

    let next_version = get_next_version(gh, PrBumpConfig::default()).await.unwrap();

    assert_eq!(next_version, Version::new(1, 2, 3));
}

#[tokio::test]
async fn pr_with_random_label_does_not_bump() {
    let mut gh = LocalGitHub::default();
    gh.add_release(Release::new("1.2.3".to_string(), ymd_midnight(2021, 1, 1)));
    gh.add_pull(PullRequest::new(
        vec!["some-label-that-means-nothing-to-semver".to_string()],
        Some(ymd_midnight(2021, 1, 2)),
    ));

    let next_version = get_next_version(gh, PrBumpConfig::default()).await.unwrap();

    assert_eq!(next_version, Version::new(1, 2, 3));
}

#[tokio::test]
async fn pr_with_one_relevant_label_and_one_random_label_bumps() {
    let mut gh = LocalGitHub::default();
    gh.add_release(Release::new("1.2.3".to_string(), ymd_midnight(2021, 1, 1)));
    gh.add_pull(PullRequest::new(
        vec![
            "some-label-that-means-nothing-to-semver".to_string(),
            "breaking".to_string(),
        ],
        Some(ymd_midnight(2021, 1, 2)),
    ));

    let next_version = get_next_version(gh, PrBumpConfig::default()).await.unwrap();

    assert_eq!(next_version, Version::new(2, 0, 0));
}

#[tokio::test]
async fn pr_with_multiple_valid_labels_bumps_the_biggest_part() {
    let mut gh = LocalGitHub::default();
    gh.add_release(Release::new("1.2.3".to_string(), ymd_midnight(2021, 1, 1)));
    gh.add_pull(PullRequest::new(
        vec![
            "fix".to_string(),
            "feature".to_string(),
            "breaking".to_string(),
        ],
        Some(ymd_midnight(2021, 1, 2)),
    ));

    let next_version = get_next_version(gh, PrBumpConfig::default()).await.unwrap();

    assert_eq!(next_version, Version::new(2, 0, 0));
}

#[tokio::test]
async fn pr_with_multiple_identical_labels_only_bumps_once() {
    let mut gh = LocalGitHub::default();
    gh.add_release(Release::new("1.2.3".to_string(), ymd_midnight(2021, 1, 1)));
    gh.add_pull(PullRequest::new(
        vec!["fix".to_string(), "fix".to_string(), "fix".to_string()],
        Some(ymd_midnight(2021, 1, 2)),
    ));

    let next_version = get_next_version(gh, PrBumpConfig::default()).await.unwrap();

    assert_eq!(next_version, Version::new(1, 2, 4));
}
