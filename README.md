## Vendor the deps

```sh
make vendor
```

Copy the instructions mentioned in the o/p to app/.cargo/config.toml
by replacing all the entries under
"### Replace the next entries based on o/p of make vendor"

## Sync the vendor files to app/vendor

```sh
make sync
```

## Test
You can run test builds of the projects to ensure offline build works.

- Clone the guest-components project

```sh
git clone https://github.com/confidential-containers/guest-components.git
```

- Clone the vendor project

```sh
git clone https://github.com/bpradipt/guest-components-vendor

```

- Checkout v0.8.0 branch

```sh
cd guest-components
git checkout -b v0.8.0-local v0.8.0
```

- Copy the `guest-components-vendor/app/.cargo/config.toml` to the project root and
adjust the `source.vendored-sources` entry

Assuming you are under `guest-components` dir, run the following command:

```sh
cp -r guest-components-vendor/app/.cargo .
```

Adjust the `source.vendored-sources` entry. For example,
if `guest-components-vendor` is under `/root/`, then the updated
entry should look like this:

```
[source.vendored-sources]
directory = "/root/guest-components-vendor/app/vendor"
```

- Run builds

```sh
cd attestation-agent
make KBC="cc_kbc" ttrpc=true LIBC=gnu
cd ..
```

```sh
cd api-server-rest
make
cd ..
```

```sh
cd confidential-data-hub
make RESOURCE_PROVIDER=kbs PROVIDER=kbs LIBC=gnu
cd ..
```
