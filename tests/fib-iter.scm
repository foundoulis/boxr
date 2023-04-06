#!/usr/bin/env boxr

(define (print-fibs a b)
  (print a)
  (print-fibs b (+ a b)))

(print-fibs 0 1)
