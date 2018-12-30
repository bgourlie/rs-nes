import("./pkg").then(module => {
  document.getElementById("rom_file_selector").addEventListener("change", e => {
    const fileReader = new FileReader();
    fileReader.addEventListener("load", e => {
      const bytes = new Uint8Array(e.target.result);
      module.load_rom(bytes);
    });

    fileReader.readAsArrayBuffer(e.target.files[0]);
  });

  module.run();
});
