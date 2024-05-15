const toggleTheme = () => {
  document.body.classList.toggle('dark-mode');
};

const setTheme = () => {
  if (window.matchMedia('(prefers-color-scheme: light)').matches) {
    document.body.classList.add('dark-mode');
  } else {
    document.body.classList.remove('dark-mode');
  }
};

// Set theme on initial load
setTheme();

// Set theme when user changes their color scheme preference
window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', setTheme);

// Toggle theme when a button is clicked
document.getElementById('theme-toggle').addEventListener('click', toggleTheme);

