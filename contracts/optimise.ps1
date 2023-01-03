$current = (("\c" + (Get-Location).tostring().substring(2)).replace('\','/'))
Invoke-Expression "docker run --rm -v ${current}:/code --mount type=volume,source='contracts_cache',target=/code/target --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry cosmwasm/rust-optimizer:0.12.10"
