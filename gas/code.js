function main(){
    const folderId = "1dxBzIZM4JLWFj0Au3AHSQUZoL4Jl1rXz"
    let bytes = test_image_gen(64,64);

    let blob = Utilities.newBlob(bytes, 'image/png');
    // フォルダを取得
    let folder = DriveApp.getFolderById(folderId);

    // ファイルをドライブに保存
    folder.createFile(blob.setName("壁紙.png"));
}