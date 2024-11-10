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
          :data="data"
          :expanded-keys="expandedKeys"
          :render-switcher-icon="renderSwitcherIcon"
          :node-props="nodeProps"
          @drop="handleDrop"
          @update:expanded-keys="handleExpandedKeysChange"
      />
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
  import { h, ref } from "vue";
  import { readDir, BaseDirectory } from '@tauri-apps/plugin-fs';
  import { open } from '@tauri-apps/plugin-dialog';
  import { NDropdown, NButton, NTree, NDivider, NIcon } from 'naive-ui';
  import type { TreeDropInfo, TreeOption } from 'naive-ui';
  import { QueueMusicRound } from '@vicons/material'
  import { AddCircleOutline as AddIcon } from '@vicons/ionicons5'
  import { EditOutlined as EditIcon } from '@vicons/antd'
  import { Delete16Regular as DeleteIcon } from '@vicons/fluent'

  const showDropdown = ref(false)
  const x = ref(0)
  const y = ref(0)
  const expandedKeysRef = ref<string[]>([])
  const dataRef = ref([{
    label: '默认播放列表',
    key: '00',
    children: []
  }])

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

  const data = dataRef;
  const expandedKeys = expandedKeysRef;

  async function openDir(): string | null {
    const path = await open({
      multiple: false,
      directory: true,
    });
    if (path) {
      const result = get_dir_children(path, 0)
    }
  }
  async function get_dir_children(path: string, index: number): Array {
    let dir = [];
    if (path) {
      const entries = await readDir(path);
      dir.push({
        label: 'a',
        key: '10',
        children: []
      })
      for (let i = 0; i < entries.length; i++) {
        let item = entries[i];
        if (item.isDirectory) {
          dir.push({
            label: item.name,
            key: item.name,
            children: []
          })
          get_dir_children(path + "/" + item.name)
        }
      }
    }
    return dir;
  }

  function renderSwitcherIcon() {
      return h(NIcon, {"size": 18 }, { default: () => h(QueueMusicRound) })
  }
  function handleExpandedKeysChange(expandedKeys: string[]) {
    expandedKeysRef.value = expandedKeys
  }
  function handleSelect(value: string) {
    console.log('select', value)
    showDropdown.value = false
  }
  function handleClickOutside() {
    showDropdown.value = false
  }
  function nodeProps({ option }: { option: TreeOption }) {
    return {
      onContextmenu(e: MouseEvent): void {
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
        dataRef.value
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
          dataRef.value
      )
      if (nodeSiblings === null || nodeIndex === null)
        return
      nodeSiblings.splice(nodeIndex, 0, dragNode)
    }
    else if (dropPosition === 'after') {
      const [nodeSiblings, nodeIndex] = findSiblingsAndIndex(
          node,
          dataRef.value
      )
      if (nodeSiblings === null || nodeIndex === null)
        return
      nodeSiblings.splice(nodeIndex + 1, 0, dragNode)
    }
    dataRef.value = Array.from(dataRef.value)
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
  function renderIcon(icon: Component) {
    return () => {
      return h(NIcon, null, {
        default: () => h(icon)
      })
    }
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
  :deep(.n-tree .n-tree-node-switcher) {
    position: relative;
    top: -2px;
  }

</style>