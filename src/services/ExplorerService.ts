import { invoke } from "@tauri-apps/api/core";
import { TreeNode } from "primevue/treenode";

class ExplorerService {
  async getExplorerTree(parentNode: TreeNode | undefined): Promise<TreeNode[]> {
    const nodes: TreeNode[] = await invoke("get_data_sources", {
      parentNodeKey: parentNode?.key,
    });
    console.log("expanding node", parentNode, nodes);
    return nodes;
  }
}

export default ExplorerService;
