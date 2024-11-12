<template>
  <div class="play-list-content">
    <div class="buttons">
      <n-button type="primary">新建歌单</n-button>
      <n-button type="primary" @click="openDir">导入文件夹</n-button>
    </div>
    <n-divider />
    <div class="play-list">
      <n-tree
          block-line
          draggable
          :data="treeData"
          :expanded-keys="expandedKeysRef"
          :render-switcher-icon="renderSwitcherIcon"
          :node-props="nodeProps"
          @drop="handleDrop"
          @update:expanded-keys="handleExpandedKeysChange">
        <template #empty>
          <span></span>
        </template>
      </n-tree>
      <n-dropdown
          trigger="manual"
          placement="bottom-start"
          :show="showDropdown"
          :options="dropdownOptions"
          :x="x"
          :y="y"
          @select="handleSelect"
          @clickoutside="handleClickOutside" />
    </div>
  </div>
</template>
<script setup lang="ts">
  import {h, onMounted, ref, reactive } from "vue";
  import { open } from '@tauri-apps/plugin-dialog';
  import { Store, load } from '@tauri-apps/plugin-store';
  import { NDropdown, NButton, NTree, NDivider, NIcon } from 'naive-ui';
  import type { TreeDropInfo, TreeOption } from 'naive-ui';
  import { QueueMusicRound } from '@vicons/material'
  import { AddCircleOutline as AddIcon } from '@vicons/ionicons5'
  import { EditOutlined as EditIcon } from '@vicons/antd'
  import { Delete16Regular as DeleteIcon } from '@vicons/fluent'
  import { invoke } from "@tauri-apps/api/core";

  let store: Store = null;
  const showDropdown = ref(false)
  const x = ref(0)
  const y = ref(0)
  const expandedKeysRef = ref<string[]>([])
  const treeData = ref<TreeOption[]>([])
  const dropdownOptions = [
    {
      label: '新增',
      key: 'add',
      icon: renderIcon(AddIcon)
    },
    {
      label: '编辑',
      key: 'edit',
      icon: renderIcon(EditIcon)
    },
    {
      label: '删除',
      key: 'delete',
      icon: renderIcon(DeleteIcon)
    }
  ]
  onMounted(async () => {
    store = await load('store.json', { autoSave: false });
    await store.clear();
    const playList = await getStoreData('play-list', store);
    if (!playList) {
      treeData.value = [{
        label: '默认播放列表',
        key: '00',
        isDirectory: true,
        path: '',
        children: []
      }];
      await setStoreData('play-list', treeData.value, store);
    } else {
      treeData.value = playList;
    }
  })

  /**
   * 打开选择目录对话框
   * */
  async function openDir(): string | null {
    const path = await open({
      multiple: false,
      directory: true,
    });
    if (path) {
      let root = {
        label: path.split('\\').pop(),
        key: 0,
        isLeaf: false,
        children: []
      }
      const result = await invoke("list_files_and_directories", { dirPath: path, initialCounter: treeData.value.slice[-1][0].key + 1});
      console.log(result);
      // 获取用户选择的目录及所有子目录和子文件
      // root.children = await getDirChildren(path, root, treeData.value.length);
      // 保存到Store中
      // await setStoreData('all-play-list', [...treeData.value, root], store);
      // 筛选数据，只获取目录
      // root.children = getDirs(root.children);
      // 更新到目录树中
      treeData.value.push(result);
    }
  }
  /**
   * 递归获取选择目录的所有子目录及文件
   * @param path 目录路径
   * @param root 根节点
   * @param index 层级
   * @returns Promise<Array> 子文件
   * */
  async function getDirChildren(path: string, root: any, index: number): Promise<Array> {
    let dir = [];
    if (path) {
      index++;
      let split = path.split('\\'); // 去掉根目录名
      let name = split.pop(); // 去掉根目录名
      const entries = await readDir(path);
      for (let i = 0; i < entries.length; i++) {
        let item = entries[i];
        let child = {
          label: item.name,
          key: root.key + index.toString() + i,
          isLeaf: item.isFile,
          children: null
        }
        if (item.isDirectory) {
          child.children = await getDirChildren(path + "\\" + item.name, child, index);
        }
        dir.push(child);
      }
    }
    return dir;
  }
  /**
   * 递归获取所有目录
   * @param data 数据
   * @returns Array<TreeOption> 所有目录
   * */
  function getDirs(data: Array<TreeOption>): Array<TreeOption> {
    let dir = [];
    if (data && Array.isArray(data)) {
      for (let item of data) {
        if (!item.isLeaf) {
          item.children = getDirs(item.children);
          dir.push(item);
        }
      }
    }
    return dir;
  }
  /**
   * 递归获取所有目录
   * @param data 数据
   * @returns Array<TreeOption> 所有目录
   * */
  function getFiles(data: Array<TreeOption>): Array<TreeOption> {
    let dir = [];
    if (data && Array.isArray(data)) {
      for (let item of data) {
        if (!item.isLeaf) {
          item.children = getFiles(item.children);
          dir.push(item);
        }
      }
    }
    return dir;
  }

  /**
   * 选中目录事件
   * @param value 选中的目录名称
   * */
  function handleSelect(value: string) {
    console.log('select', value)
    showDropdown.value = false
  }
  /**
   * 点击空白处隐藏操作目录弹出框
   * */
  function handleClickOutside() {
    showDropdown.value = false
  }
  function handleExpandedKeysChange(expandedKeys: string[]) {
    expandedKeysRef.value = expandedKeys;
  }
  /**
   * 播放列表目录点击右键弹出操作框
   * @param node 点击的节点
   * */
  function nodeProps({ option }: { option: TreeOption }) {
    return {
      onContextmenu(e: MouseEvent): void {
        console.log('contextmenu', option)
        showDropdown.value = true;
        x.value = e.clientX
        y.value = e.clientY
        e.preventDefault()
      }
    }
  }
  function handleDrop({ node, dragNode, dropPosition }: TreeDropInfo) {
    const [dragNodeSiblings, dragNodeIndex] = findSiblingsAndIndex(
        dragNode,
        treeData.value
    )
    if (dragNodeSiblings === null || dragNodeIndex === null)
      return
    dragNodeSiblings.splice(dragNodeIndex, 1)
    if (dropPosition === 'inside') {
      if (node.children) {
        node.children.unshift(dragNode)
      }
      else {
        node.children = [dragNode]
      }
    }
    else if (dropPosition === 'before') {
      const [nodeSiblings, nodeIndex] = findSiblingsAndIndex(
          node,
          treeData.value
      )
      if (nodeSiblings === null || nodeIndex === null)
        return
      nodeSiblings.splice(nodeIndex, 0, dragNode)
    }
    else if (dropPosition === 'after') {
      const [nodeSiblings, nodeIndex] = findSiblingsAndIndex(
          node,
          treeData.value
      )
      if (nodeSiblings === null || nodeIndex === null)
        return
      nodeSiblings.splice(nodeIndex + 1, 0, dragNode)
    }
    treeData.value = Array.from(treeData.value)
  }

  function findSiblingsAndIndex(
      node: TreeOption,
      nodes?: TreeOption[]
  ): [TreeOption[], number] | [null, null] {
    if (!nodes)
      return [null, null]
    for (let i = 0; i < nodes.length; ++i) {
      const siblingNode = nodes[i]
      if (siblingNode.key === node.key)
        return [nodes, i]
      const [siblings, index] = findSiblingsAndIndex(node, siblingNode.children)
      if (siblings && index !== null)
        return [siblings, index]
    }
    return [null, null]
  }
  /**
   * 目录前面的图标
   * */
  function renderSwitcherIcon(value) {
    return h(NIcon, {"size": 18 }, { default: () => h(QueueMusicRound) })
  }
  /**
   * 新增、修改、删除前面的图标
   * */
  function renderIcon(icon: Component) {
    return () => {
      return h(NIcon, null, {
        default: () => h(icon)
      })
    }
  }
  /**
   * 存储键值对到Store中
   * @param key 键
   * @param value 值
   * @param store
   * */
  async function setStoreData(key: string, value: any, store: Store) {
    await store.set(key, value);
    await store.save();
  }
  /**
   * 从Store中获取值
   * @param key 键
   * @param store
   * */
  async function getStoreData(key: string, store: Store) {
    return await store.get(key);
  }
</script>

<style scoped>
  .play-list-content {
    width: 100%;
    height: 100%;
    position: absolute;
    left: 0;
    top: 0;
  }
  .buttons {
    width: 100%;
    height: 40px;
  }
  .play-list {
    width: 100%;
    height: calc(100% - 89px)
  }
  :deep(.n-tree .n-tree-node-switcher) {
    position: relative;
    top: -2px;
  }

</style>