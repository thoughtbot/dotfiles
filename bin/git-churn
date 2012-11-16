#!/usr/bin/ruby
#
# Credits: Corey Haines, Gary Bernhardt, Dan Croak
#
# Put this on your $PATH (~/bin is recommended).
#
# Show churn for the files changed in the branch:
#
#   $ git-churn
#

class GitChurn
  attr_accessor :all_files_with_churn_count, :changed_files, :result

  def calculate
    get_all_files_with_churn_count
    get_changed_files
    filter_into_result
    print_result
  end

  private

  def get_all_files_with_churn_count
    self.all_files_with_churn_count =
      `git log --all #{format_logs} | #{remove_blank_lines} | #{sort_by_ascending_churn_count}`.split("\n")
  end

  def format_logs
    "--name-only --format='format:'"
  end

  def remove_blank_lines
    "grep -v '^$'"
  end

  def sort_by_ascending_churn_count
    'sort | uniq -c | sort'
  end

  def get_changed_files
    self.changed_files =
      `git log origin/master..HEAD #{format_logs} | #{remove_blank_lines} | #{remove_duplicates}`.split("\n")
  end

  def remove_duplicates
    'sort | uniq'
  end

  def filter_into_result
    self.result = all_files_with_churn_count.select { |file| file_changed?(file) }.join("\n")
  end

  def file_changed?(file)
    changed_files.any? { |changed_file| file.include?(changed_file) }
  end

  def print_result
    puts result
  end
end

git_churn = GitChurn.new
git_churn.calculate
