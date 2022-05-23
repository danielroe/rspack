import { promises as fs } from "fs"
import path from "path"

import less from "less"

import { ExternalObject, resolveFile } from "@rspack/binding"

import type { RspackPlugin } from "../index"

function resolve(baseDir: string, importPath: string) {
  let haserr = false;
  let res;
  if (
    importPath.substring(0, 1) == '/' ||
    importPath.substring(0, 2) == './' ||
    importPath.substring(0, 3) == '../'
  ) {
    res = resolveFile(baseDir, importPath);
  } else {
    try {
      res = resolveFile(baseDir, importPath);
    } catch (ex) {
      haserr = true;
    }
    if (haserr) {
      res = resolveFile(baseDir, `./${importPath}`);
    }
  }
  return res;
}


export default class LessAliasesPlugin {
  public currentDir: string;
  public callbackError: Function;
  public rspack: ExternalObject<any>

  constructor(currentDir: string, callbackError: Function, rspack: ExternalObject<any>) {
    this.callbackError = callbackError;
    this.currentDir = currentDir;
    this.rspack = rspack;
  }

  install(less: typeof import("less"), pluginManager: any) {
    let { currentDir, callbackError, rspack } = this;

    class AliasPlugin extends less.FileManager {
      loadFile(
        filename: string,
        currentDirectory: string,
        options: Record<string, unknown>,
        environment: Less.Environment
      ) {
        let resolved = undefined;
        try {
          let baseFile: string = currentDirectory
            ? currentDirectory
            : path.dirname(currentDir);
          resolved = resolve(baseFile, filename);
        } catch (err: any) {
          callbackError(err);
          return Promise.reject(err);
        }
        return super.loadFile(
          resolved ?? filename,
          currentDirectory,
          options,
          environment
        );
      }
    }
    pluginManager.addFileManager(new AliasPlugin());
  }
}

interface LessPluginOptions {
  paths?: string[]
  root?: string
}

export const LessPlugin = (options: LessPluginOptions): RspackPlugin => {
  return {
    async onLoad ({ id }, rspack) {
      const callbackError = (err: Error) => {
        console.log(err);
      }
      
      if (id.endsWith(".less")) {
        const content = await fs.readFile(id, "utf8")
        const renderResult = await less.render(content, {
          paths: [
            ...(options?.paths || ['node_modules']),
            ...(options?.root ? [options.root] : []),
          ],
          plugins: [new LessAliasesPlugin(id, callbackError, rspack)]
        })

        return {
          content: renderResult.css,
          loader: "css"
        }
      }

      return null
    },
    async onResolve(context) {}
  }
}