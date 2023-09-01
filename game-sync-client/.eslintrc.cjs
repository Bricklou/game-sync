module.exports = {
  env: {
    browser: true,
    es2021: true,
  },
  extends: ["plugin:vue/vue3-recommended", "prettier"],
  overrides: [],
  parser: "vue-eslint-parser",
  parserOptions: {
    parser: "@typescript-eslint/parser",
    ecmaVersion: "latest",
    sourceType: "module",
    vueFeatures: {
      filter: true,
      interpolationAsNonHTML: false,
    },
  },
  plugins: ["vue", "prettier"],
  rules: {
    "prettier/prettier": "error",
  },
};
