type Options = {
  multiline?: boolean;
};

export const empower = (element: HTMLDivElement, options: Options): () => void => {
  if (isEmpowered(element)) return () => {};

  addLineBreakIfNeeded(element, options);

  const handleEnter = (event: KeyboardEvent) => {
    // console.log('Before:', element.innerHTML);

    if (event.key === "Enter") {
      event.preventDefault();

      if (!options.multiline) return;

      const html = element.innerHTML;

      let value = html ? "\n" : "\n\n";

      insertTextAtCursor(value);
    }

    // console.log('After:', element.innerHTML);
  };

  const handlePaste = (event: ClipboardEvent) => {
    event.preventDefault();

    const text = event.clipboardData?.getData("text/plain") ?? "";

    insertTextAtCursor(text);

    text && element.dispatchEvent(new InputEvent("input", { bubbles: true }));
  };

  element.addEventListener("keydown", handleEnter);
  element.addEventListener("paste", handlePaste);

  return () => {
    element.removeEventListener("keydown", handleEnter);
    element.removeEventListener("paste", handlePaste);
  };
};

const breakLine = (): HTMLBRElement => document.createElement("br");

const isEmpowered = (element: HTMLDivElement): boolean => {
  const status = element.dataset.empowered === "true";

  !status && (element.dataset.empowered = "true");

  return status;
};

const addLineBreakIfNeeded = (element: HTMLDivElement, options: Options) => {
  const last = element.lastChild as any;

  let exit = false;

  exit ||= !options.multiline;
  exit ||= !last;
  exit ||= last.tagname === "BR";

  if (exit) return;

  element.appendChild(breakLine());
};

const insertTextAtCursor = (text: string) => {
  const sel = window.getSelection();

  if (!sel || sel.rangeCount === 0) return;

  const range = sel.getRangeAt(0);

  range.deleteContents();

  const lines = text.split("\n");
  const frag = document.createDocumentFragment();

  lines.forEach((line, index) => {
    if (index > 0) frag.appendChild(breakLine());
    frag.appendChild(document.createTextNode(line));
  });

  const lastChild = frag.lastChild!;
  range.insertNode(frag);

  range.setStartAfter(lastChild);
  range.collapse(true);

  sel.removeAllRanges();
  sel.addRange(range);
};
