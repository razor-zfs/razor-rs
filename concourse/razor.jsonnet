function() {
  local concourse = import '../../statehub-concourse/jsonnet/concourse.libsonnet',
  
  local rep_name = "razor",
  
  local resource_types = [
    concourse.ResourceType("fluentd", "registry.gitlab.com/replixio/automation/statehub-concourse", "fluentd-concourse-type-image"),
  ],
  
  local resources = [
    concourse.Fluentd(),
    concourse.GitResource("razor-rs", "https://gitlab.com/replix/razor-rs.git", branch="main"),
    concourse.DockerResource(rep_name+'-image', "registry.gitlab.com/replix/razor-rs", "main", auth = true),
  ],
  
  local build() = concourse.Job("build-razor-container", plan=[
      concourse.Get('razor-rs', trigger = true),
    ]
    +
    (concourse.Build('razor-rs', rep_name+'-image', extra_params={
      BUILD_ARG_GITLAB_TOKEN: "((git.password))",
      BUILD_ARG_GITLAB_USER: "((git.username))"
    }))),
  
  local jobs = [
    build(),
  ],
  
    display: concourse.display,
    resource_types: resource_types,
    resources: resources,
    jobs: jobs
}
