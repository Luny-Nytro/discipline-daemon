/**
 * This script resolves import aliases "/CompiledCode" directory to relative 
 * paths.
 * 
 * Import aliases are defined in "/tsconfig.json", however, this script
 * doesn't read that file, as aliases are copied into this script below, so, 
 * this script MUST be updated whenever the import aliases in "/tsconfig.json"
 * change.
 */

import { join, fromFileUrl } from "https://deno.land/std@0.224.0/path/mod.ts";

type Alias = {
  name: string
  path: string
}

async function resolveImportAliasesInDirectory(
  aliases: Alias[],
  directoryPath: string, 
  directoryDepth: number,
) {
  for await (const entry of Deno.readDir(directoryPath)) {
    if (entry.isFile) {
      await resolveImportAliasesInFile(
        aliases, 
        directoryPath, 
        directoryDepth, 
        entry.name,
      )
      continue
    } 
    
    if (entry.isDirectory) {
      await resolveImportAliasesInDirectory(
        aliases,
        join(directoryPath, entry.name),
        directoryDepth + 1,
      )
      continue
    }

    console.warn("Found a symlink. Cannot handle symlinks yet.")
  }
}

function isJavaScriptFileName(fileName: string) {
  return fileName.endsWith(".js")
}

function isTypeScriptDeclarationFileName(fileName: string) {
  return fileName.endsWith(".d.ts")
}

async function resolveImportAliasesInFile(
  aliases: Alias[],
  directoryPath: string, 
  directoryDepth: number,
  fileName: string,
) {
  if (
    !isTypeScriptDeclarationFileName(fileName) 
    && 
    !isJavaScriptFileName(fileName)
  ) {
    return
  }

  const filePath = join(directoryPath, fileName)
  const currentFileContent = await Deno.readTextFile(filePath)
  let newFileContent = currentFileContent

  for (const alias of aliases) {
    newFileContent = resolveImportAliasInFile(alias, directoryDepth, newFileContent)
  }

  if (newFileContent !== currentFileContent) {
    await Deno.writeTextFile(filePath, newFileContent);
    console.log(`Updated: ${filePath}`);
  }
}

function resolveImportAliasInFile(
  alias: Alias, 
  directoryDepth: number,
  declarationFileContent: string,
) {
  // matches an "import" keyword, followed by anthing stopping before a quote,
  // which is the start of the import specifier, then matches a quoted "alias.name"
  // where the openning quote is the same type (single or double) as the closing
  // quote.
  // 
  // This will produce these match groups
  //  - 1st: contains the "import" keyword followed by whatever goes before the import specifier.
  //  - 2nd: the import specifier openinng quote.
  //  - 3rd: the alias name
  //  - 4th: the import specifier closing quote.
  const regex = new RegExp(`(import[^'"]+)(['"])(${alias.name})(\\2)`, 'gu');
  
  const path = directoryDepth === 0
    ? `./${alias.path}`
    : `../`.repeat(directoryDepth) + alias.path

  return declarationFileContent.replace(regex, (_, beginning, quote) => {
    return `${beginning}${quote}${path}${quote}`
  })
}
// Conversts import aliases to relative paths.
//
// This is expected to be called the on the directory 
// containing all typescript declaration files of your 
// package.
//
// Only works for import aliases that refer to the current
// package.
export async function go(
  declarationsDirectoryPath: string,
  aliases: Alias[]
) {
  await resolveImportAliasesInDirectory(aliases, declarationsDirectoryPath, 0)
}


const compiledCodeDirectoryUrl = new URL('../CompiledCode', import.meta.url);
const compiledCodeDirectoryPath = fromFileUrl(compiledCodeDirectoryUrl);

await go(compiledCodeDirectoryPath, [{ 
  name: "@Pkg/Option", 
  path: "ElementaryTypes/Option" 
}, { 
  name: "@Pkg/Tried", 
  path: "ElementaryTypes/Tried" 
}, { 
  name: "@Pkg/Duration", 
  path: "ChronicTypes/Duration" 
}, { 
  name: "@Pkg/DateTime", 
  path: "ChronicTypes/DateTime" 
}, { 
  name: "@Pkg/Weekday", 
  path: "ChronicTypes/Weekday" 
}, { 
  name: "@Pkg/WeekdayRange", 
  path: "ChronicTypes/WeekdayRange" 
}, { 
  name: "@Pkg/Hour", 
  path: "ChronicTypes/Hour" 
}, { 
  name: "@Pkg/Minute", 
  path: "ChronicTypes/Minute" 
}, { 
  name: "@Pkg/Second", 
  path: "ChronicTypes/Second" 
}, { 
  name: "@Pkg/Month", 
  path: "ChronicTypes/Month" 
}, { 
  name: "@Pkg/MonthDay", 
  path: "ChronicTypes/MonthDay" 
}, { 
  name: "@Pkg/CountdownTimer", 
  path: "ChronicTypes/CountdownTimer" 
}, { 
  name: "@Pkg/Time", 
  path: "ChronicTypes/Time" 
}, { 
  name: "@Pkg/TimeRange", 
  path: "ChronicTypes/TimeRange" 
}, { 
  name: "@Pkg/Meridiem", 
  path: "ChronicTypes/Meridiem" 
}, { 
  name: "@Pkg/Integer", 
  path: "ElementaryTypes/Integer/mod"
}, { 
  name: "@Pkg/OperatingSystemPassword", 
  path: "CommonTypes/OperatingSystemPassword" 
}, { 
  name: "@Pkg/OperatingSystemUsername", 
  path: "CommonTypes/OperatingSystemUsername" 
}, { 
  name: "@Pkg/ByPasswordEnabler", 
  path: "CommonTypes/ByPasswordEnabler" 
}, { 
  name: "@Pkg/Password", 
  path: "CommonTypes/Password" 
}, { 
  name: "@Pkg/Uuid", 
  path: "ElementaryTypes/Uuid" 
}, { 
  name: "@Pkg/JsonSerde", 
  path: "ElementaryTypes/JsonSerde" 
}, { 
  name: "@Pkg/Display", 
  path: "ElementaryTypes/Display" 
}, { 
  name: "@Pkg/Executer", 
  path: "Executer" 
}, { 
  name: "@Pkg/UserAccess", 
  path: "Features/UserAccess/mod"
}, { 
  name: "@Pkg/ShadowVaults", 
  path: "Features/ShadowVaults/mod"
}, { 
  name: "@Pkg/NetworkingAccess", 
  path: "Features/NetworkingAccess/mod"
}, {
  name: "@Pkg/Unique",
  path: "ElementaryTypes/Unique"
}])