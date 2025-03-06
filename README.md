# Issued

## 1. bevy_web_assets, s3 CDN load failed (main branch)
It is stored in S3 CDN using the bevy_web_assets library.

https://assets.choiyunseok.com/models/base_character_girl.glb 

I'm trying to load data, but

```
2025-03-06T01:56:00.367248Z ERROR bevy_asset::server:
An I/O error occurred while loading the asset. Unexpected status code 500 while loading https://assets.choiyunseok.com/models/base_character_girl.glb:
Head byte length must be less than 8kb.
```

When you access via browser, the file will be downloaded.


## 2. Static model cannot be loaded from wasm when deployed to S3 cdn  (assets_server Branch)

Loading the model from the S3 CDN fails every time, and as a workaround, when loading the model static from the assets folder and running viewer.html after building, it works normally, but.

Upload the entire built file to s3 CDN, https://wasm.choiyunseok.com/avatar/viewer.html 
The model does not load when connected to


```
Normal run - apps/avatar /wasm/viewer.html
Model load not possible - apps/avatar/wasm upload to s3 -> https://wasm.choiyunseok.com/avatar/viewer.html, The model does not load.
```
