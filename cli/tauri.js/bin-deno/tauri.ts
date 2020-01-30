const cmds = ['init', 'dev', 'build', 'help', 'icon', 'info']
const cmd = Deno.args[0]

import { createRequire } from 'https://deno.land/std/node/module.ts'
import { process } from 'https://deno.land/std/node/process.ts'
const require_ = createRequire(import.meta.url)
;(window as any).process = process

/**
 * @description This is the bootstrapper that in turn calls subsequent
 * Tauri Commands
 *
 * @param {string|array} command
 */
const tauri = async function(
  command: string[] | string
): Promise<boolean | void> {
  if (typeof command === 'object') {
    // technically we just care about an array
    command = command[0]
  }

  if (
    !command ||
    command === '-h' ||
    command === '--help' ||
    command === 'help'
  ) {
    console.log(`
    Description
      This is the Tauri CLI.
    Usage
      $ tauri ${cmds.join('|')}
    Options
      --help, -h     Displays this message
      --version, -v  Displays the Tauri CLI version
    `)
    Deno.exit(0)
    // TODO: is this necessary in Deno?
    return false // do this for node consumers and tests
  }

  if (command === '-v' || command === '--version') {
    // console.log((await import('package.json')).version)
    // TODO: is this necessary in Deno?
    return false // do this for node consumers and tests
  }

  if (cmds.includes(command)) {
    // TODO: what does this do?
    // if (process.argv && !process.env.test) {
    //   process.argv.splice(2, 1)
    // }
    console.log(`[tauri]: running ${command}`)
    require_(`../bin/tauri-${command}`)
  } else {
    console.log(`Invalid command ${command}. Use one of ${cmds.join(',')}.`)
  }
}

tauri(cmd).catch((e: Error) => {
  throw e
})
