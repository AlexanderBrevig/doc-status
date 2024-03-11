#!/usr/bin/env bash

interval="$(jq -c '.interval' statuses.json)"
jq -c '.files|to_entries[]|[.key,.value.timestamp,.value.status]|@tsv' statuses.json \
 | tr -d '"' > statuses.tsv
sed -i 's/\\t/	/g' statuses.tsv
while IFS=$'\t' read -r filename timestamp status ; do
  timenow="$(date +%s)"
  limit="$((timenow - interval))"

  if [[ "$timestamp" -lt "$limit" ]]; then
    jq ".files[\"$filename\"].status = false" statuses.json > tmp.json
    mv -- tmp.json statuses.json
    echo "$filename is outdated status = $status"
  fi
done < statuses.tsv
rm statuses.tsv
