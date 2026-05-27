#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")" && pwd)"
AWK="$ROOT/awk_crystal"
REF=$(command -v awk)

# ---- env setup for build deps (skip if already set) ----
if [ -z "${LIBRARY_PATH-}" ] || ! echo "$LIBRARY_PATH" | grep -q pcre; then
  export PATH="/tmp/crystal/bin:$PATH"
  export LIBRARY_PATH="/tmp/pcre-dev/usr/lib/x86_64-linux-gnu:/tmp/event-dev/usr/lib/x86_64-linux-gnu${LIBRARY_PATH:+:$LIBRARY_PATH}"
  export LD_LIBRARY_PATH="/tmp/pcre-dev/usr/lib/x86_64-linux-gnu:/tmp/event-dev/usr/lib/x86_64-linux-gnu${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
fi

PASS=0
FAIL=0
TOTAL=0

GREEN='\033[32m'; RED='\033[31m'; CYAN='\033[36m'; GREY='\033[90m'; NC='\033[0m'

header()  { echo -e "\n${CYAN}━━━ $1 ━━━${NC}"; }
ok()      { PASS=$((PASS+1)); TOTAL=$((TOTAL+1)); echo -e "  ${GREEN}✓${NC} $1"; }
fail()    { FAIL=$((FAIL+1)); TOTAL=$((TOTAL+1)); echo -e "  ${RED}✗${NC} $1"; echo -e "${GREY}    expected: $2${NC}"; echo -e "${GREY}    got:      $3${NC}"; }

run_awk()      { "$REF" "$@" 2>/dev/null || true; }
run_crystal()  { "$AWK" "$@" 2>/dev/null || true; }

assert_eq() {
  local desc="$1" prog="$2" input="${3:-}" label="${4:-}"
  local expect actual
  expect=$(printf "%b" "$input" | run_awk "$prog" || true)
  actual=$(printf "%b" "$input" | run_crystal "$prog" || true)
  if [ "$expect" = "$actual" ]; then
    ok "$desc${label:+ ($label)}"
  else
    fail "$desc${label:+ ($label)}" "$expect" "$actual"
  fi
}

assert_eq_noinput() {
  local label=""; [ $# -ge 3 ] && label="$3"
  assert_eq "$1" "$2" "" "$label"
}

assert_file() {
  local desc="$1" prog="$2" file="$3"
  local expect actual
  expect=$(run_awk "$prog" "$file"); actual=$(run_crystal "$prog" "$file")
  if [ "$expect" = "$actual" ]; then ok "$desc"; else fail "$desc" "$expect" "$actual"; fi
}

DATA="$ROOT/../tests/test_input.txt"
CSV="$ROOT/../tests/test2.csv"

# =====================================================================
header "1. Print"
assert_eq_noinput 'print no args'  'BEGIN{print}'
assert_eq_noinput 'print string'   'BEGIN{print "hello"}'
assert_eq_noinput 'print number'   'BEGIN{print 42}'
assert_eq_noinput 'print two args' 'BEGIN{print 1, 2}'
assert_eq_noinput 'print three'    'BEGIN{print "a","b","c"}'
assert_eq_noinput 'print empty'    'BEGIN{print ""}'

# =====================================================================
header "2. Fields"
assert_file 'print $0'   '{print}'      "$DATA"
assert_file 'print $1'   '{print $1}'   "$DATA"
assert_file 'print $2'   '{print $2}'   "$DATA"
assert_file 'print $1,$3' '{print $1, $3}' "$DATA"
assert_file 'NF'         '{print NF}'   "$DATA"
assert_file 'NR'         '{print NR}'   "$DATA"
assert_file 'NR,$1'      '{print NR, $1}' "$DATA"
assert_file '$NF'        '{print $NF}'  "$DATA"
assert_file '$1 $2'      '{print $1, $2}' "$DATA"
assert_file '$0 with NF' '{print $0, NF}' "$DATA"
assert_file 'multiple stmts' '{print $1; print $2}' "$DATA"

# =====================================================================
header "3. BEGIN / END + Patterns"
assert_eq_noinput 'BEGIN only'   'BEGIN{print "start"}'
assert_eq_noinput 'END only'     'END{print "end"}'               "hello"
assert_eq_noinput 'BEGIN + END'  'BEGIN{print "s"} END{print "e"}' ""
assert_file 'regex pattern'      '/Smith/{print}'                 "$DATA"
assert_file 'regex field'        '/Smith/{print $1, $2}'          "$DATA"
assert_file 'BEGIN pattern line' 'BEGIN{print "x"} {print NR} END{print "y"}' "$DATA"
assert_eq 'expr pattern true'  '1 {print "y"}' "a"
assert_eq 'expr pattern false' '0 {print "n"}' "a"

# =====================================================================
header "4. Variables & Arithmetic"
assert_eq_noinput 'assign int'   'BEGIN{x=42; print x}'
assert_eq_noinput 'assign str'   'BEGIN{x="hello"; print x}'
assert_eq_noinput 'assign field' '{x=$2; print x}' "$(echo 'a b c')"
assert_eq_noinput 'add'          'BEGIN{print 2+3}'
assert_eq_noinput 'sub'          'BEGIN{print 5-3}'
assert_eq_noinput 'mul'          'BEGIN{print 4*3}'
assert_eq_noinput 'div'          'BEGIN{print 7/2}'
assert_eq_noinput 'mod'          'BEGIN{print 10%3}'
assert_eq_noinput 'pow'          'BEGIN{print 2^3}'
assert_eq_noinput 'precedence'   'BEGIN{print 2+3*4}'
assert_eq_noinput 'parens'       'BEGIN{print (2+3)*4}'
assert_eq_noinput '+= '          'BEGIN{x=5; x+=3; print x}'
assert_eq_noinput '-= '          'BEGIN{x=5; x-=3; print x}'
assert_eq_noinput '*= '          'BEGIN{x=5; x*=3; print x}'
assert_eq_noinput '/= '          'BEGIN{x=6; x/=2; print x}'
assert_eq_noinput '%= '          'BEGIN{x=10; x%=3; print x}'
assert_eq_noinput '++ post'      'BEGIN{x=1; print x++; print x}'
assert_eq_noinput '++ pre'       'BEGIN{x=1; print ++x}'
assert_eq_noinput '-- post'      'BEGIN{x=2; print x--; print x}'
assert_eq_noinput '-- pre'       'BEGIN{x=2; print --x}'

# =====================================================================
header "5. Field Separator"
assert_file 'FS default'     '{print $1}'      "$CSV"
assert_eq 'FS assignment'    'BEGIN{FS=","} {print $2}' "a,b,c"
echo -n "  "
EXPECT=$(run_awk -F, '{print $2}' <<< "x,y,z")
ACTUAL=$(run_crystal -F, '{print $2}' <<< "x,y,z")
if [ "$EXPECT" = "$ACTUAL" ]; then ok "-F flag"; else fail "-F flag" "$EXPECT" "$ACTUAL"; fi

# =====================================================================
header "6. If / Else"
assert_eq_noinput 'if true'     'BEGIN{if (1) print "y"}'
assert_eq_noinput 'if false'    'BEGIN{if (0) print "n"}'
assert_eq_noinput 'if/else'     'BEGIN{if (0) print "y"; else print "n"}'
assert_eq_noinput 'else if'     'BEGIN{x=2; if(x==1) print "a"; else if(x==2) print "b"}'
assert_eq_noinput 'nested if'   'BEGIN{if (1) {if (1) print "y"}}'
assert_eq_noinput 'if block'    'BEGIN{if (1) {print "a"; print "b"}}'
assert_file 'if field'          '{if ($3 > 60000) print $1, $3}' "$DATA"

# comparisons — parens avoid redirect ambiguity with mawk
assert_eq_noinput 'eq'  'BEGIN{print (1==1)}'
assert_eq_noinput 'ne'  'BEGIN{print (1!=2)}'
assert_eq_noinput 'lt'  'BEGIN{print (1<2)}'
assert_eq_noinput 'gt'  'BEGIN{print (2>1)}'
assert_eq_noinput 'le'  'BEGIN{print (1<=1)}'
assert_eq_noinput 'ge'  'BEGIN{print (2>=2)}'

assert_eq_noinput 'and' 'BEGIN{print (1&&0)}'
assert_eq_noinput 'or'  'BEGIN{print (0||1)}'
assert_eq_noinput 'not' 'BEGIN{print !0}'
assert_eq_noinput 'ternary' 'BEGIN{print 1?"y":"n"}'
assert_eq_noinput 'ternary field' '{print ($3>60000)?"high":"low"}' "$(echo 'x 70000')"

# =====================================================================
header "7. Loops"
assert_eq_noinput 'while'      'BEGIN{i=3; while(i>0){print i; i--}}'
assert_eq_noinput 'for'        'BEGIN{for(i=1;i<=3;i++) print i}'
assert_eq_noinput 'break'      'BEGIN{for(i=1;i<=5;i++){if(i>2) break; print i}}'
assert_eq_noinput 'continue'   'BEGIN{for(i=1;i<=5;i++){if(i==3) continue; print i}}'
assert_eq_noinput 'next'       '{if(NR==2) next; print}' "$(printf 'a\nb\nc')"
assert_eq_noinput 'exit'       'BEGIN{exit}' "$(printf 'a\nb')"

echo -n "  "
RC=0
"$AWK" 'BEGIN{exit 42}' /dev/null 2>/dev/null && RC=$? || RC=$?
if [ "$RC" = "42" ]; then ok "exit code 42"; else fail "exit code 42" "42" "$RC"; fi

# =====================================================================
header "8. Built-in String Functions"
assert_eq_noinput 'length'          'BEGIN{print length("hello")}'
assert_eq_noinput 'length noarg'    '{print length()}'              "$(echo 'hello')"
assert_eq_noinput 'substr'          'BEGIN{print substr("hello",2,3)}'
assert_eq_noinput 'substr to end'   'BEGIN{print substr("hello",4)}'
assert_eq_noinput 'index'           'BEGIN{print index("hello","ll")}'
assert_eq_noinput 'index miss'      'BEGIN{print index("hello","xyz")}'
assert_eq_noinput 'toupper'         'BEGIN{print toupper("abc")}'
assert_eq_noinput 'tolower'         'BEGIN{print tolower("ABC")}'
assert_eq_noinput 'match'           'BEGIN{print match("hello","ll"); print RSTART, RLENGTH}'
assert_eq_noinput 'match miss'      'BEGIN{print match("hello","xyz"); print RSTART, RLENGTH}'
assert_eq_noinput 'sub'             'BEGIN{s="hello"; sub(/l/,"x",s); print s}'
assert_eq_noinput 'gsub'            'BEGIN{s="hello"; gsub(/l/,"x",s); print s}'
assert_eq_noinput 'sub $0'          'BEGIN{$0="hello"; sub(/l/,"x"); print}'
assert_eq_noinput 'gsub $0'         'BEGIN{$0="hello"; gsub(/l/,"x"); print}'
assert_eq_noinput 'sprintf'         'BEGIN{s=sprintf("%d-%s",42,"abc"); print s}'

# split: compare ref count + iterate our own way (for-in order varies by impl)
echo -n "  "
EXPECT=$(run_awk 'BEGIN{n=split("a,b,c",a,","); print n}' < /dev/null)
ACTUAL=$(run_crystal 'BEGIN{n=split("a,b,c",a,","); print n}' < /dev/null)
if [ "$EXPECT" = "$ACTUAL" ]; then ok "split count"; else fail "split count" "$EXPECT" "$ACTUAL"; fi

# =====================================================================
header "9. Arrays"
assert_eq_noinput 'set/get'       'BEGIN{a["x"]=1; print a["x"]}'
assert_eq_noinput 'str key'       'BEGIN{a[1]="x"; print a[1]}'
assert_eq_noinput 'unset'         'BEGIN{print a["x"]}'
assert_eq_noinput 'delete elem'   'BEGIN{a["x"]=1; delete a["x"]; print a["x"]}'
assert_eq_noinput 'delete all'    'BEGIN{a["x"]=1; delete a; print a["x"]}'

# for-in order varies by awk impl — just check keys are present
echo -n "  "
OUT=$(run_crystal 'BEGIN{a[1]="x"; a[2]="y"; for(k in a) print k,a[k]}' < /dev/null)
if echo "$OUT" | grep -q "1 x" && echo "$OUT" | grep -q "2 y"; then ok "for-in contains both keys"; else fail "for-in contains both keys" "1 x and 2 y" "$OUT"; fi

echo -n "  "
OUT=$(run_crystal 'BEGIN{a[1]="x"; delete a[1]; for(k in a) print k}' < /dev/null)
if [ -z "$OUT" ]; then ok "for-in after delete"; else fail "for-in after delete" "empty" "$OUT"; fi

# =====================================================================
header "10. Printf"
assert_eq_noinput 'printf string'  'BEGIN{printf "%s", "hello"}'
assert_eq_noinput 'printf int'     'BEGIN{printf "%d", 42}'
assert_eq_noinput 'printf float'   'BEGIN{printf "%f", 3.14}'
assert_eq_noinput 'printf newline' 'BEGIN{printf "%s\n", "a"}'
assert_eq_noinput 'printf %%'      'BEGIN{printf "%%"}'
assert_eq_noinput 'printf width'   'BEGIN{printf "|%10s|", "hi"}'
assert_eq_noinput 'printf left'    'BEGIN{printf "|%-10s|", "hi"}'
assert_eq_noinput 'printf multi'   'BEGIN{printf "%d %s %d", 1, "x", 2}'

# =====================================================================
echo ""
echo -e "${CYAN}══════════════════════════════════════════${NC}"
echo -e "  ${TOTAL} tests  ·  ${GREEN}${PASS} passed${NC}  ·  ${RED}${FAIL} failed${NC}"
echo -e "${CYAN}══════════════════════════════════════════${NC}"
[ "$FAIL" -eq 0 ] || exit 1
