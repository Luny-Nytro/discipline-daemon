// import * as esbuild from "https://deno.land/x/esbuild@v0.23.1/mod.js";
import { bundle } from "https://deno.land/x/emit@0.40.0/mod.ts";

export async function build(source: string, destination: string) {
  const built = await bundle(source, {
    importMap: {
      imports: {
        "Pkg": "./discipline/mod.ts"
      }
    },
    allowRemote: true,
    type: "module",
    minify: true,
  })

  Deno.writeTextFileSync(destination, built.code)
  // await esbuild.build({
  //   entryPoints: [source],
  //   bundle: true,
  //   outfile: destination,
  //   minify: false,
  //   platform: "browser", // Use "node" if targeting Node.js environment
  //   target: ["es2024"], // Specify the ECMAScript version target
  //   format: "esm", // Use "esm" for ES Modules or "cjs" for CommonJS
  //   sourcemap: true, // Include sourcemaps
  //   alias: {
  //     "Pkg": "./discipline/mod.ts"
  //   }
  // })
  
  // await esbuild.stop()
}