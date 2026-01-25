export default function Grid({
  children,
  columns = 5,
  gap = 0,
  width = "1fr",
  wrap = false,
  className = ""
}: {
  children: React.ReactNode
  columns?: number
  gap?: string | number
  width?: string
  wrap?: boolean
  className?: string
}) {
  return (
    <div
      className={className}
      style={{
        display: "grid",
        gridTemplateColumns: wrap
          ? `repeat(auto-fit, minmax(${width}, 1fr))`
          : `repeat(${columns}, ${width})`,
        gap
      }}
    >
      {children}
    </div>
  )
}
