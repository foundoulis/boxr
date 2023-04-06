#!/usr/bin/env boxr

(define (sum-first-five x)
  (match_list x
    ((a b c d e) (+ a b c d e))
    ((a b c d) (+ a b c d))
    ((a b c) (+ a b c))
    ((a b) (+ a b))
    ((a) a)
    (NIL 0)))

(print (sum-first-five '(1 2 3 4)))

(print (sum-first-five '(1 2 3)))

(print (sum-first-five '(1 2)))

(print (sum-first-five '(1)))

(print (sum-first-five '()))
