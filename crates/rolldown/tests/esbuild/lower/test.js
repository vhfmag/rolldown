const fs = require("fs");
const path = require("path");

const lines = fs.readFileSync("../pass.txt").toString();
const linesArray = lines.split("\n");
console.log(`linesArray: `, linesArray);
// Function to rename directories
function renameDirectoriesSync(baseDir) {
	// Read the contents of the base directory
	try {
		const files = fs.readdirSync(baseDir);

		// Process each item in the directory
		files.forEach((file) => {
			const filePath = path.join(baseDir, file);

			// Check if it is a directory and starts with '.'
			const stats = fs.statSync(filePath);

			if (stats.isDirectory() && file.startsWith(".") && linesArray.includes(file.slice(1))) {
				const newFileName = file.substring(1);
				const newFilePath = path.join(baseDir, newFileName);

				try {
				    fs.renameSync(filePath, newFilePath);
				    console.log(`Renamed ${filePath} to ${newFilePath}`);
				} catch (err) {
				    console.error(`Error renaming directory ${filePath}:`, err);
				}
			}
		});
	} catch (err) {
		console.error("Error reading directory:", err);
	}
}

// Specify the base directory
const baseDir = ".";

// Call the function to rename directories
renameDirectoriesSync(baseDir);
