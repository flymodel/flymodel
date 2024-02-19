POST http://localhost:9009/upload/model-version-artifact

[MultipartFormData]
file: file,model_artifact.json; application/octet-stream;
artifact: file,model_artifact_payload.json; application/json;


HTTP 200

[Asserts]
jsonpath "$.blob" == 2
jsonpath "$.id" == 1
jsonpath "$.name" == "Test model artifact.json"
jsonpath "$.version_id" == 1


GET http://localhost:9009/download/model-version-artifact?artifact_id=1

HTTP 200

[Asserts]
header "Content-Type" contains "application/json"
header "Content-Disposition" contains "attachment; filename=\"Test model artifact.json\""
header "Digest" contains "sha256=8507c46253d708fef9fab62512c6912858cbf734ad2797ac93d7209501c0060e"

sha256 == hex,8507c46253d708fef9fab62512c6912858cbf734ad2797ac93d7209501c0060e;

jsonpath "$.training_rounds" == 2000
jsonpath "$.acc" == 0.9887
jsonpath "$.loss" == 0.0034

