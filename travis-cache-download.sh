#!/bin/sh

set -e

url="$1"
dir="${HOME}/.cache/rust"
file="${dir}/$(basename "${url}")"
header="${file}.header"

if [ -f "${file}" -a -f "${header}" ]; then
	etag="$(grep -i '^etag: ' "${header}"  2> /dev/null | sed 's/^etag: //i')"
fi

mkdir -p "${dir}"
tmpfile="$(mktemp "${file}.XXXXXXXXX")"
tmpheader="$(mktemp "${header}.XXXXXXXXX")"
curl -sL ${etag:+-H "If-None-Match: ${etag}"} --dump-header "${tmpheader}" "$url" --output "${tmpfile}" || {
	rm -f "${tmpfile}" "${tmpheader}"
}
mv "${tmpfile}" "${file}"
mv "${tmpheader}" "${header}"
