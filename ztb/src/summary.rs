pub(super) fn summary(pool: impl AsRef<str>) -> anyhow::Result<String> {
    Ok(pool.as_ref().to_string())
}
