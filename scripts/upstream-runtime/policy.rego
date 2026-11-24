package clauseforge
default allow := false
allow if { input.role == "amendment-reviewer"; input.affected_controls <= 80 }
