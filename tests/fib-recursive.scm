#!/usr/bin/env boxr
(define (fib n)
  (if (< n 2)
      n
      (+ (fib (- n 1)) (fib (- n 2)))))

(define (print-fibs start)
  (print (fib start))
  (print-fibs (+ 1 start)))

(print-fibs 0)
