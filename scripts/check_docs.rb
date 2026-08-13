# frozen_string_literal: true

require "open3"
require "optparse"
require "pathname"

REQUIRED = %w[
  README.md README.zh-CN.md CHANGELOG.md CODE_OF_CONDUCT.md
  CODE_OF_CONDUCT.zh-CN.md LICENSE
  CONTRIBUTING.md CONTRIBUTING.zh-CN.md SECURITY.md SECURITY.zh-CN.md
  SUPPORT.md SUPPORT.zh-CN.md docs/MATURITY.md docs/MATURITY.zh-CN.md
  docs/RELEASE_CHECKLIST.md docs/RELEASE_CHECKLIST.zh-CN.md
  docs/product_spec.md docs/product_spec.zh-CN.md
].freeze
FORBIDDEN_SCAFFOLDS = %w[index.html skills docs/implementation_plan.md].freeze
UTF8_BOM = "\xEF\xBB\xBF".b.freeze

options = { root: Dir.pwd }
OptionParser.new { |parser| parser.on("--root PATH") { |path| options[:root] = path } }.parse!

root = Pathname.new(File.expand_path(options[:root]))
errors = []
tracked_output, tracked_error, tracked_status = Open3.capture3("git", "-C", root.to_s, "ls-files", "-z")
abort("git ls-files failed: #{tracked_error.strip}") unless tracked_status.success?
tracked = tracked_output.split("\0").reject(&:empty?)

REQUIRED.each do |relative|
  errors << "missing required file: #{relative}" unless tracked.include?(relative) && root.join(relative).file?
end

FORBIDDEN_SCAFFOLDS.each do |relative|
  prefix = "#{relative}/"
  errors << "forbidden public scaffold: #{relative}" if tracked.any? { |path| path == relative || path.start_with?(prefix) }
end

tracked.sort.each do |relative|
  path = root.join(relative)
  next unless path.file?
  bytes = path.binread
  next if bytes.include?("\0")

  errors << "UTF-8 BOM is forbidden: #{relative}" if bytes.start_with?(UTF8_BOM)
  text = bytes.force_encoding(Encoding::UTF_8)
  errors << "invalid UTF-8: #{relative}" unless text.valid_encoding?
end

tracked.grep(/\.md\z/).sort.each do |relative|
  path = root.join(relative)
  next unless path.file?
  text = path.read(encoding: "UTF-8")
  text.scan(/\[[^\]]*\]\(([^)]+)\)/).flatten.each do |target|
    target = target.split(/\s+/, 2).first.delete_prefix("<").delete_suffix(">")
    next if target.empty? || target.start_with?("#", "https://", "http://", "mailto:")

    local = target.split("#", 2).first
    next if local.empty?
    resolved = path.dirname.join(local).cleanpath
    errors << "broken local link in #{relative}: #{target}" unless resolved.exist?
  end
end

readme = root.join("README.md")
chinese_readme = root.join("README.zh-CN.md")
if readme.file? && !readme.read(encoding: "UTF-8").include?("README.zh-CN.md")
  errors << "README.md must link to README.zh-CN.md"
end
if chinese_readme.file? && !chinese_readme.read(encoding: "UTF-8").include?("README.md")
  errors << "README.zh-CN.md must link to README.md"
end

if errors.empty?
  puts "Documentation contracts passed (#{tracked.length} tracked files scanned)."
  exit 0
end

warn errors.join("\n")
exit 1
