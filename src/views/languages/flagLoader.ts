const flags = import.meta.glob('../../assets/flags/*.png', { eager: true });

const flagMap: Record<string, string> = {};
for (const path in flags) {
  const fileName = path.split('/').pop()?.split('.')[0]; 
  if (fileName) {
    debugger; // This will pause the execution in the browser's debugger
    const flagCode = fileName?.split('_')[1].toLowerCase(); 
    flagMap[flagCode] = (flags[path] as { default: string }).default;
  }
}

export default flagMap;