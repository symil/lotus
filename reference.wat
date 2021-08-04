;; Local variable
(local $name i32) ;; Declare a local variable, must be declared before any instruction
(local.set $name (i32.const 42)) ;; Set local variable
(local.get $name) ;; Retrieve local variable (push its value on the stack)

;; Memory
(i32.store (i32.const 46) (i32.const 100)) ;; write value `100` at address `46`
(i32.load (i32.const 46)) ;; read value at address `46`

;; Loops
(block ;; start of block A
    (block ;; start of block B
        ;; some instructions
        br 1 ;; jumps to `end of block A`, because block A is `1` level ahead of the current block
        br 0 ;; jumps to `end of block B`, bcause block B is `0` level ahead of the current block (it is the current block)
    
        ;; end of block B
    )
    ;; end of block A
)

(block ;; start of block A
    (loop ;; start of loop B
        ;; some instructions
        br 1 ;; jumps to `end of block A`, because block A is `1` level ahead of the current block
        br 0 ;; jumps to `start of loop B`, bcause loop B is `0` level ahead of the current block (it is the current block), and it is a loop
    
        ;; end of block B
    )
    ;; end of block A
)