module.exports = {
  purge: [],
  darkMode: false, // or 'media' or 'class'
  theme: {
    container: {
      center: true,
    },
    extend: {
      boxShadow: {
        "green-400": "0 6px 20px 0 rgba(52, 211, 153, 0.3)",
        "green-500": "0 6px 20px 0 rgba(16, 185, 129, 0.3)",
        "green-600": "0 6px 20px 0 rgba(5, 150, 105, 0.3)",
      },
    },
  },
  variants: {
    extend: {
      backgroundColor: ["active", "focus"],
      boxShadow: ["active", "focus"],
    },
  },
  plugins: [require("@tailwindcss/typography")],
};
