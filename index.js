const wasm = import('./pkg');

document.getElementById("file_input")
    .addEventListener('change', render_molecule);

function render_molecule(){
    let file = this.files[0];
    let reader = new FileReader();
    reader.readAsText(file);

    reader.onload = function() {
        wasm.then(m => m.render_molecule(reader.result))
            .catch(alert);
    };

    reader.onerror = function() {
        alert(reader.error);
    };
}
