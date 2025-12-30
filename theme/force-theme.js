// Force Rust theme - remove any stored theme preference
(function() {
    // Set the theme to rust
    localStorage.setItem('mdbook-theme', 'rust');

    // Apply the rust class to html element
    document.documentElement.classList.remove('light', 'coal', 'navy', 'ayu');
    document.documentElement.classList.add('rust');
})();

