package clauseforge
default allow := false
allow if {
  input.role == "amendment-reviewer"
  input.supplier != ""
  input.amendment != ""
  input.affected_controls <= 80
  input.cure_days > 0
  input.cure_days <= 365
}
