# frozen_string_literal: true

require "fileutils"
require "minitest/autorun"
require "open3"
require "rbconfig"
require "tmpdir"

class CheckDocsTest < Minitest::Test
  CHECKER = File.expand_path("check_docs.rb", __dir__)
  REQUIRED = %w[
    README.md README.zh-CN.md CHANGELOG.md CODE_OF_CONDUCT.md
    CODE_OF_CONDUCT.zh-CN.md LICENSE
    CONTRIBUTING.md CONTRIBUTING.zh-CN.md SECURITY.md SECURITY.zh-CN.md
    SUPPORT.md SUPPORT.zh-CN.md docs/MATURITY.md docs/MATURITY.zh-CN.md
    docs/RELEASE_CHECKLIST.md docs/RELEASE_CHECKLIST.zh-CN.md
    docs/product_spec.md docs/product_spec.zh-CN.md
  ].freeze

  def test_valid_repository_passes
    with_repository do |root|
      result = run_checker(root)
      assert result[:status].success?, result[:output]
    end
  end

  def test_missing_translation_fails
    with_repository(remove: ["SECURITY.zh-CN.md"]) do |root|
      result = run_checker(root)
      refute result[:status].success?
      assert_includes result[:output], "missing required file: SECURITY.zh-CN.md"
    end
  end

  def test_broken_local_link_fails
    with_repository(overrides: { "README.md" => "[简体中文](missing.md)\n" }) do |root|
      result = run_checker(root)
      refute result[:status].success?
      assert_includes result[:output], "broken local link in README.md: missing.md"
    end
  end

  def test_legacy_public_scaffold_fails
    with_repository(overrides: { "skills/mcp-tools.json" => "{}\n" }) do |root|
      result = run_checker(root)
      refute result[:status].success?
      assert_includes result[:output], "forbidden public scaffold: skills"
    end
  end

  private

  def with_repository(remove: [], overrides: {})
    Dir.mktmpdir("json-yaml-swiss-docs-") do |root|
      files = REQUIRED.to_h { |path| [path, "# #{File.basename(path)}\n"] }
      files["README.md"] = "[简体中文](README.zh-CN.md)\n"
      files["README.zh-CN.md"] = "[English](README.md)\n"
      files.merge!(overrides)
      remove.each { |path| files.delete(path) }
      files.each do |path, content|
        target = File.join(root, path)
        FileUtils.mkdir_p(File.dirname(target))
        File.binwrite(target, content)
      end
      run_git(root, "init", "--quiet")
      run_git(root, "add", "--all")
      yield root
    end
  end

  def run_checker(root)
    stdout, stderr, status = Open3.capture3(RbConfig.ruby, CHECKER, "--root", root)
    { output: stdout + stderr, status: status }
  end

  def run_git(root, *arguments)
    _stdout, stderr, status = Open3.capture3("git", "-C", root, *arguments)
    raise stderr unless status.success?
  end
end
