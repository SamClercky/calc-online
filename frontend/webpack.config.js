const path = require('path');
// @ts-ignore
const MiniCssExtractPlugin = require('mini-css-extract-plugin');
// @ts-ignore
const HtmlWebpackPlugin = require('html-webpack-plugin');
// @ts-ignore
const CopyWebpack = require("copy-webpack-plugin");
// @ts-ignore
const sass = require("svelte-preprocess-sass").sass;

const mode = process.env.NODE_ENV || 'development';
const prod = mode === 'production';

const packageJson = require("./package.json");

// ---------------------------
// ----- M E T A D A T A -----
// ---------------------------
const NAME = "HCalc";
const VERSION = packageJson.version;
const AUTHOR = packageJson.author;
// ---------------------------
// ----- M E T A D A T A -----
// ---------------------------

module.exports = {
  entry: "./src/bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "[name].js",
    chunkFilename: "[name].[id].js"
  },
  resolve: {
		extensions: ['.mjs', '.js', '.svelte', '.wasm', ".ts"]
	},
  mode,
  plugins: [
    new MiniCssExtractPlugin({
		filename: '[name].css'
	}),
	new HtmlWebpackPlugin({
		template: "./public/index.html",
		title: NAME,
		favicon: "./public/favicon.png"
	}),
  ],
  module: {
		rules: [
			{
				test: /\.svelte$/,
				exclude: /node_modules/,
				use: {
					loader: 'svelte-loader',
					options: {
						emitCss: true,
						hotReload: true,
						style: sass()
					}
				}
			},
			{
				test: /\.css$/,
				use: [
					/**
					 * MiniCssExtractPlugin doesn't support HMR.
					 * For developing, use 'style-loader' instead.
					 * */
					prod ? MiniCssExtractPlugin.loader : 'style-loader',
					'css-loader'
				]
			},
			// all files with a `.ts` or `.tsx` extension will be handled by `ts-loader`
			{ test: /\.tsx?$/, loader: "ts-loader" }
		]
  },
  devtool: prod ? false: 'source-map'
};
