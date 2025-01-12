### GRPCURL

```
grpcurl -plaintext -proto ./proto/calculator.proto -d '{"a": 2, "b": 3}' '[::1]:50001' calculator.Calculator.Add
```

```
grpcurl -emit-defaults -plaintext '[::1]:50001' calculator.Admin.GetRequestCount

grpcurl -H "Authorization: Bearer some-secret-token" -emit-defaults -plaintext '[::1]:50001' calculator.Admin.GetRequestCount
```

#### After adding tonic_reflection

```
grpcurl -plaintext  -d '{"a": 2, "b": 3}' '[::1]:50001' calculator.Calculator.Add
```

## NOTES

### OUT_DIR

The folder in which all output and intermediate artifacts should be placed. This folder is inside the build directory for the package being built, and it is unique for the package in question.
