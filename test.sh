while true
do
{ git ls-files; git ls-files . --exclude-standard --others; } | entr -d cargo test
done
