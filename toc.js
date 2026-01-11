// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="intro.html">Introduction</a></li><li class="chapter-item expanded "><a href="crash_course.html">Crash Course on Computing</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Part I: Calculator</li><li class="chapter-item expanded "><a href="01_calculator/calc_intro.html">Calculator</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="01_calculator/grammar_lexer_parser.html">Grammar, Lexer and Parser</a></li><li class="chapter-item expanded "><a href="01_calculator/ast.html">Abstract Syntax Tree (AST) and Interpreter</a></li><li class="chapter-item expanded "><a href="01_calculator/jit_intro.html">Just-In-Time (JIT) Compiler with LLVM</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="01_calculator/basic_llvm.html">Basic Example</a></li><li class="chapter-item expanded "><a href="01_calculator/ast_traversal.html">AST Traversal Patterns</a></li></ol></li><li class="chapter-item expanded "><a href="01_calculator/vm.html">Virtual Machine (VM), Bytecode and Interpreter</a></li><li class="chapter-item expanded "><a href="01_calculator/repl.html">Read-Eval-Print Loop (REPL)</a></li><li class="chapter-item expanded "><a href="01_calculator/debugging.html">Debugging Your Language</a></li><li class="chapter-item expanded "><a href="01_calculator/exercise.html">Exercises</a></li></ol></li><li class="chapter-item expanded "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Part II: Firstlang (Interpreted)</li><li class="chapter-item expanded "><a href="transition_1_to_2.html">From Calculator to Real Language</a></li><li class="chapter-item expanded "><a href="02_firstlang/intro.html">Firstlang: Your First Real Language</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="02_firstlang/syntax.html">Python-like Syntax</a></li><li class="chapter-item expanded "><a href="02_firstlang/variables.html">Variables and Assignment</a></li><li class="chapter-item expanded "><a href="02_firstlang/functions.html">Functions</a></li><li class="chapter-item expanded "><a href="02_firstlang/control_flow.html">Control Flow: If/Else and While</a></li><li class="chapter-item expanded "><a href="02_firstlang/recursion.html">Recursion</a></li><li class="chapter-item expanded "><a href="02_firstlang/repl.html">Building the REPL</a></li><li class="chapter-item expanded "><a href="02_firstlang/fibonacci.html">Computing Fibonacci</a></li><li class="chapter-item expanded "><a href="02_firstlang/exercises.html">Exercises</a></li></ol></li><li class="chapter-item expanded "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Part III: Secondlang (Compiled)</li><li class="chapter-item expanded "><a href="transition_2_to_3.html">From Interpreted to Compiled</a></li><li class="chapter-item expanded "><a href="03_secondlang/intro.html">Secondlang: Adding Types and Compilation</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="03_secondlang/why_types.html">Why Types Matter</a></li><li class="chapter-item expanded "><a href="03_secondlang/annotations.html">Type Annotations</a></li><li class="chapter-item expanded "><a href="03_secondlang/inference.html">Type Inference</a></li><li class="chapter-item expanded "><a href="03_secondlang/optimizations.html">AST Optimizations (Visitor Pattern)</a></li><li class="chapter-item expanded "><a href="03_secondlang/ir.html">From AST to IR</a></li><li class="chapter-item expanded "><a href="03_secondlang/codegen.html">LLVM Code Generation</a></li><li class="chapter-item expanded "><a href="03_secondlang/jit_fibonacci.html">JIT Compiling Fibonacci</a></li><li class="chapter-item expanded "><a href="03_secondlang/exercises.html">Exercises</a></li></ol></li><li class="chapter-item expanded "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Part IV: Thirdlang (Object-Oriented)</li><li class="chapter-item expanded "><a href="transition_3_to_4.html">From Functions to Objects</a></li><li class="chapter-item expanded "><a href="04_thirdlang/intro.html">Thirdlang: Adding Classes and Objects</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="04_thirdlang/why_classes.html">Why Classes?</a></li><li class="chapter-item expanded "><a href="04_thirdlang/classes_syntax.html">Class Syntax and Parsing</a></li><li class="chapter-item expanded "><a href="04_thirdlang/constructors.html">Constructors and Object Creation</a></li><li class="chapter-item expanded "><a href="04_thirdlang/methods.html">Methods and Self</a></li><li class="chapter-item expanded "><a href="04_thirdlang/memory.html">Memory Management</a></li><li class="chapter-item expanded "><a href="04_thirdlang/codegen_classes.html">LLVM Code Generation for Classes</a></li><li class="chapter-item expanded "><a href="04_thirdlang/optimization.html">Optimizing LLVM IR</a></li><li class="chapter-item expanded "><a href="04_thirdlang/running.html">Running Thirdlang</a></li><li class="chapter-item expanded "><a href="04_thirdlang/exercises.html">Exercises</a></li></ol></li><li class="chapter-item expanded "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Beyond the Book</li><li class="chapter-item expanded "><a href="whats_next.html">What&#39;s Next</a></li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded affix "><li class="part-title">Appendices</li><li class="chapter-item expanded "><a href="testing.html">Testing Your Language</a></li><li class="chapter-item expanded "><a href="glossary.html">Glossary</a></li><li class="chapter-item expanded "><a href="resources.html">Resources</a></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
