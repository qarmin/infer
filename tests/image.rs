extern crate infer;

use std::fs;
use infer::Infer;

#[test]
fn test_jpg() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("image/jpeg"), 
        ext: String::from("jpg"),
    }, 
    info.get(&fs::read("src/testdata/sample.jpg").unwrap()).unwrap());
}

#[test]
fn test_png() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("image/png"), 
        ext: String::from("png"),
    }, 
    info.get(&fs::read("src/testdata/sample.png").unwrap()).unwrap());
}

#[test]
fn test_gif() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("image/gif"), 
        ext: String::from("gif"),
    }, 
    info.get(&fs::read("src/testdata/sample.gif").unwrap()).unwrap());
}

#[test]
fn test_tif() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("image/tiff"), 
        ext: String::from("tif"),
    }, 
    info.get(&fs::read("src/testdata/sample.tif").unwrap()).unwrap());
}

#[test]
fn test_tif2() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("image/tiff"), 
        ext: String::from("tif"),
    }, 
    info.get(&fs::read("src/testdata/sample2.tif").unwrap()).unwrap());
}

#[test]
fn test_tif3() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("image/tiff"), 
        ext: String::from("tif"),
    }, 
    info.get(&fs::read("src/testdata/sample3.tif").unwrap()).unwrap());
}

#[test]
fn test_tif4() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("image/tiff"), 
        ext: String::from("tif"),
    }, 
    info.get(&fs::read("src/testdata/sample4.tif").unwrap()).unwrap());
}

#[test]
fn test_tif5() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("image/tiff"), 
        ext: String::from("tif"),
    }, 
    info.get(&fs::read("src/testdata/sample5.tif").unwrap()).unwrap());
}

#[test]
fn test_bmp() {
    let info = Infer::new();
    
    assert_eq!(infer::Type { 
        mime: String::from("image/bmp"), 
        ext: String::from("bmp"),
    }, 
    info.get(&fs::read("src/testdata/sample.bmp").unwrap()).unwrap());
}

#[test]
fn test_psd() {
    let info = Infer::new();
    
    assert_eq!(infer::Type { 
        mime: String::from("image/vnd.adobe.photoshop"), 
        ext: String::from("psd"),
    }, 
    info.get(&fs::read("src/testdata/sample.psd").unwrap()).unwrap());
}

#[test]
fn test_ico() {
    let info = Infer::new();

    assert_eq!(infer::Type { 
        mime: String::from("image/x-icon"), 
        ext: String::from("ico"),
    }, 
    info.get(&fs::read("src/testdata/sample.ico").unwrap()).unwrap());
}


#[test]
fn test_heif() {
    let info = Infer::new();
    
    assert_eq!(infer::Type { 
        mime: String::from("image/heif"), 
        ext: String::from("heif"),
    }, 
    info.get(&fs::read("src/testdata/sample.heic").unwrap()).unwrap());
}