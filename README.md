### Untar bug repro

Problem:
A file should show up after untarring, but instead, it appears as a directory.

Repro'ed on both mac 10.14.1 and docker rust:1.30-stretch

#### Step 1
```
cargo run --bin rust-tar tar
```
This will produce (it is hardcoded to tar up `test_data/hello.txt`)
```
archive.tar.gz
```


Using the system tar to check the content:
```
tar -tvf archive.tar.gz
-rw-r--r-- 0/0              11 2018-11-14 08:35 gen/scrooge/297e53822ddf/wilyns.thrift.src.main.thrift.thrift-scala/c8959dfdc97a/com/twitter/wilyns/thriftscala/LookupRequest.scala
```
looks great as it is a file, not a directory.

#### Step 2
Now, use `rust-tar` to untar it to `/tmp/123/`

```
rm -rf /tmp/123 && cargo run --bin rust-tar untar archive.tar.gz /tmp/123
```

The listings show up:
```
$ find /tmp/123
/tmp/123
/tmp/123/gen
/tmp/123/gen/scrooge
/tmp/123/gen/scrooge/297e53822ddf
/tmp/123/gen/scrooge/297e53822ddf/wilyns.thrift.src.main.thrift.thrift-scala
/tmp/123/gen/scrooge/297e53822ddf/wilyns.thrift.src.main.thrift.thrift-scala/c8959dfdc97a
/tmp/123/gen/scrooge/297e53822ddf/wilyns.thrift.src.main.thrift.thrift-scala/c8959dfdc97a/com
/tmp/123/gen/scrooge/297e53822ddf/wilyns.thrift.src.main.thrift.thrift-scala/c8959dfdc97a/com/twitter
/tmp/123/gen/scrooge/297e53822ddf/wilyns.thrift.src.main.thrift.thrift-scala/c8959dfdc97a/com/twitter/wilyns
/tmp/123/gen/scrooge/297e53822ddf/wilyns.thrift.src.main.thrift.thrift-scala/c8959dfdc97a/com/twitter/wilyns/thriftscala
/tmp/123/gen/scrooge/297e53822ddf/wilyns.thrift.src.main.thrift.thrift-scala/c8959dfdc97a/com/twitter/wilyns/thriftscala/LookupRequest.scala

```

However, `LookupRequest.scala` is a directory!
```
file /tmp/123/gen/scrooge/297e53822ddf/wilyns.thrift.src.main.thrift.thrift-scala/c8959dfdc97a/com/twitter/wilyns/thriftscala/LookupRequest.scala
/tmp/123/gen/scrooge/297e53822ddf/wilyns.thrift.src.main.thrift.thrift-scala/c8959dfdc97a/com/twitter/wilyns/thriftscala/LookupRequest.scala: directory
```

### Observation

Now, apply the patch in the repro, then repeat step 1 & 2. The issue does not appear.
