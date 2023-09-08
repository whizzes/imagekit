<div>
  <div align="center">
    <img
      alt="ImageKit Logo"
      src="https://raw.githubusercontent.com/EstebanBorai/imagekit/main/assets/imagekit.png"
      height="52"
      width="250"
    />
  </div>
  <h4 align="center">
    Rust API Client for ImageKit.io a file storage and image processing
    service
  </h4>
</div>

<div align="center">

  [![Crates.io](https://img.shields.io/crates/v/imagekit.svg)](https://crates.io/crates/imagekit)
  [![Documentation](https://docs.rs/imagekit/badge.svg)](https://docs.rs/imagekit)
  ![Build](https://github.com/EstebanBorai/imagekit/workflows/build/badge.svg)
  ![Clippy](https://github.com/EstebanBorai/imagekit/workflows/clippy/badge.svg)
  ![Formatter](https://github.com/EstebanBorai/imagekit/workflows/fmt/badge.svg)
  ![Tests](https://github.com/EstebanBorai/imagekit/workflows/test/badge.svg)

</div>

## Usage

You must retrieve your Public and Private Keys from the
[ImageKit Developer Options][1].

Then create an instance of `ImageKit` and initialize the client.

```rust
use imagekit::ImageKit;
use imagekit::management::Details;
use imagekit::delete::Delete;
use imagekit::upload::types::FileType;
use imagekit::upload::{Options, Upload, UploadFile};
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut image_kit = ImageKit::new(
        "your_public_api_key",
        "your_private_api_key",
        "https://ik.imagekit.io/your_imagekit_id/",
    );

    // Upload an image from File
    let file = File::open("assets/ferris.jpeg").await.unwrap();
    let opts = Options::new(upload_file, "ferris");
    let upload_result = imagekit.upload(opts).await.unwrap();

    // Retrieve details from a given file, in this case the file we just uploaded
    let details_result = imagekit.upload(opts).await.unwrap();

    // Delete a file
    let delete_result = imagekit.delete(upload_result.file_id).await;
}
```

## Features

The main goal of this crate is to support the main three functionalities
provided by ImageKit. URL Generation, File Upload and File Management.

The following list, provides a closer view to supported features and planned
features which are not yet implemented. Feel free to contribute by opening
an issue, pull request or discussion.

- [x] URL Generation
  - [x] Basic Image Resizing
  - [ ] Crop, Crop Modes and Focus
  - [ ] Commonly Used Transformations
- [ ] Signed URL Generation
- [x] File Upload ([File Upload API][2])
  - [x] From `tokio::fs::File` (Binary)
  - [ ] From `std::fs::File` (Binary)
  - [ ] From URL
  - [ ] From Base64
- [ ] File Management
  - [x] List Files
  - [x] Search Files
  - [x] Get File Details
  - [ ] Get File Versions
  - [ ] Get File Metadata
  - [ ] Custom Metadata Fields
    - [ ] Create
    - [ ] List
    - [ ] Update
    - [ ] Delete
  - [x] Delete File
  - [ ] Update File Details
  - [ ] Tags
    - [ ] Bulk Addition
    - [ ] Bulk Deletion
  - [ ] AI Tags
    - [ ] Bulk Deletion
  - [ ] Delete File Version
  - [ ] Bulk Delete Files
  - [ ] Copy File
  - [ ] Move File
  - [ ] Rename File
  - [ ] Restore File Version
  - [ ] Folders
    - [ ] Create
    - [ ] Copy
    - [ ] Delete
    - [ ] Move
  - [ ] Bulk Job Status
  - [ ] Cache
   - [ ] Purge

> If you notice theres missing features in this list, please open an issue or PR.

## Release

In order to create a release you must push a Git tag as follows

```sh
git tag -a <version> -m <message>
```

**Example**

```sh
git tag -a v0.1.0 -m "First release"
```

> Tags must follow semver conventions
> Tags must be prefixed with a lowercase `v` letter.

Then push tags as follows:

```sh
git push origin main --follow-tags
```

## Contributing

Every contribution to this project is welcome. Feel free to open a pull request,
an issue or just by starting this project.

## License

As most Rust projects, this crate is licensed under both, the Apache License
and the MIT License.

[1]: https://imagekit.io/dashboard/developer/api-keys
[2]: https://docs.imagekit.io/api-reference/upload-file-api/server-side-file-upload
