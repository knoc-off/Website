document.addEventListener('DOMContentLoaded', function() {
  const themeToggle = document.getElementById('theme-toggle');
  const body = document.body;

  // Check if a theme preference is stored in local storage
  const savedTheme = localStorage.getItem('theme');
  if (savedTheme === 'dark') {
    body.classList.add('dark-mode');
    themeToggle.classList.add('dark-mode');
  }

  // Add event listener to the theme toggle button
  themeToggle.addEventListener('click', function() {
    body.classList.toggle('dark-mode');
    themeToggle.classList.toggle('dark-mode');

    // Save the current theme preference to local storage
    if (body.classList.contains('dark-mode')) {
      localStorage.setItem('theme', 'dark');
    } else {
      localStorage.setItem('theme', 'light');
    }
  });
});

