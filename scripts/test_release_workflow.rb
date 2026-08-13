# frozen_string_literal: true

require "minitest/autorun"

class ReleaseWorkflowTest < Minitest::Test
  WORKFLOW = File.expand_path("../.github/workflows/release.yml", __dir__)

  def test_rust_setup_installs_components_required_by_release_checks
    workflow = File.read(WORKFLOW, encoding: "UTF-8")
    setup_start = workflow.index("      - name: Set up Rust\n")
    checks_start = workflow.index("      - name: Run release checks\n")

    refute_nil setup_start, "release workflow must contain the Rust setup step"
    refute_nil checks_start, "release workflow must contain the release checks step"
    assert_operator setup_start, :<, checks_start

    setup_step = workflow[setup_start...checks_start]
    assert_includes setup_step,
                    "rustup component add rustfmt clippy --toolchain 1.95.0"
  end
end
