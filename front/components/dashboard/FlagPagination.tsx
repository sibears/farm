"use client"

import { Button } from "@/components/ui/button"

type FlagPaginationProps = {
  currentPage: number
  totalPages: number
  onPageChange: (page: number) => void
  showingFrom: number
  showingTo: number
  totalCount: number
  isAtEnd?: boolean
}

export function FlagPagination({
  currentPage,
  totalPages,
  onPageChange,
  showingFrom,
  showingTo,
  totalCount,
  isAtEnd = false,
}: FlagPaginationProps) {
  if (totalPages <= 1) return null

  return (
    <div className="flex items-center justify-between pt-4">
      <div className="flex items-center space-x-2">
        <p className="text-sm font-mono text-muted-foreground">
          Showing {showingFrom} to {showingTo} of {totalCount} results
        </p>
      </div>

      <div className="flex items-center space-x-2">
        <Button
          variant="outline"
          size="sm"
          onClick={() => onPageChange(1)}
          disabled={currentPage === 1}
          className="font-mono bg-transparent border-border"
        >
          <span>⇤</span>
        </Button>
        <Button
          variant="outline"
          size="sm"
          onClick={() => onPageChange(Math.max(currentPage - 1, 1))}
          disabled={currentPage === 1}
          className="font-mono bg-transparent border-border"
        >
          <span>←</span>
        </Button>

        <div className="flex items-center space-x-1">
          {Array.from({ length: Math.min(5, totalPages) }, (_, i) => {
            let pageNum: number
            if (totalPages <= 5) {
              pageNum = i + 1
            } else if (currentPage <= 3) {
              pageNum = i + 1
            } else if (currentPage >= totalPages - 2) {
              pageNum = totalPages - 4 + i
            } else {
              pageNum = currentPage - 2 + i
            }

            return (
              <Button
                key={pageNum}
                variant={currentPage === pageNum ? "default" : "outline"}
                size="sm"
                onClick={() => onPageChange(pageNum)}
                className={`font-mono w-8 h-8 p-0 ${
                  currentPage === pageNum ? "bg-primary text-primary-foreground" : "bg-transparent border-border"
                }`}
              >
                {pageNum}
              </Button>
            )
          })}
        </div>

        <Button
          variant="outline"
          size="sm"
          onClick={() => onPageChange(currentPage + 1)}
          disabled={isAtEnd}
          className="font-mono bg-transparent border-border"
        >
          <span>→</span>
        </Button>
        <Button
          variant="outline"
          size="sm"
          onClick={() => onPageChange(totalPages)}
          disabled={isAtEnd}
          className="font-mono bg-transparent border-border"
        >
          <span>⇥</span>
        </Button>
      </div>
    </div>
  )
}
