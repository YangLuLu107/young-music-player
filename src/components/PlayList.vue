<template>
  <div class="play-list-content">
    <div class="buttons">
      <n-button type="primary">导入文件夹</n-button>
    </div>
    <div class="play-list">
      <n-tree
          block-line
          draggable
          :data="data"
          :expanded-keys="expandedKeys"
          @drop="handleDrop"
          @update:expanded-keys="handleExpandedKeysChange"
      />
    </div>
  </div>
</template>
<script setup lang="ts">
  import { ref } from "vue";
  import { repeat } from 'seemly'
  import { NButton,  NTree } from 'naive-ui';
  import type { TreeDropInfo, TreeOption } from 'naive-ui';

  const expandedKeysRef = ref<string[]>([])
  const checkedKeysRef = ref<string[]>([])
  const dataRef = ref(createData() || [])

  const data = dataRef;
  const expandedKeys = expandedKeysRef;
  const checkedKeys = checkedKeysRef;
  function handleExpandedKeysChange(expandedKeys: string[]) {
    expandedKeysRef.value = expandedKeys
  }
  function handleCheckedKeysChange(checkedKeys: string[]) {
    checkedKeysRef.value = checkedKeys
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

  function createData(level = 4, baseKey = ''): TreeOption[] | undefined {
    if (!level)
      return undefined
    return repeat(6 - level, undefined).map((_, index) => {
      const key = `${baseKey}${level}${index}`
      return {
        label: createLabel(level),
        key,
        children: createData(level - 1, key)
      }
    })
  }

  function createLabel(level: number): string {
    if (level === 4)
      return '道生一'
    if (level === 3)
      return '一生二'
    if (level === 2)
      return '二生三'
    if (level === 1)
      return '三生万物'
    return ''
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
  }
</style>