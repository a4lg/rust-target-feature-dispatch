#! /bin/sh
if test ! -f target_arch.txt; then
    echo 'ERROR: target_arch.txt not found on the current directory.' 1>&2
    exit 1
fi
{
    cat target_arch.txt |
        grep '^[^#].*' |
        sed 's/.*$/"\0" || /;$s/ || $//' |
        tr -d '\n'
    echo ''
} >target_arch.rs
