class Kfmtbox < Formula
  desc "A Rust-based command-line utility that wraps text in flower box style comments"
  homepage "https://github.com/koftwentytwo/kfmtbox"
  url "https://github.com/koftwentytwo/kfmtbox/archive/v0.1.0.tar.gz"
  sha256 "" # This will be filled in after the first release
  license "MIT"
  head "https://github.com/koftwentytwo/kfmtbox.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    # Test basic functionality
    assert_match "kfmtbox", shell_output("#{bin}/kfmtbox --help")
    
    # Test with a simple input
    test_input = "Hello, world!"
    expected_output = shell_output("echo '#{test_input}' | #{bin}/kfmtbox --filetype rs")
    assert_match "╭", expected_output
    assert_match "Hello, world!", expected_output
  end
end