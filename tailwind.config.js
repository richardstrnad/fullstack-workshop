const colors = require('tailwindcss/colors')

/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./frontend/input.css",
    "./frontend/src/**/*.{rs,html,css}",
    "./frontend/dist/**/*.html",
  ],
  theme: {
    extend: {
		  colors: {
			  primary: colors.blue,
              secondary: colors.pink,
			  base: colors.slate
		  },
	  },
  },
  plugins: [
    require('@tailwindcss/forms'),
  ],
}

