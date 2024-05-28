export default function Tags({ tags, points }) {
  return <>
    <div className="tags">
      {tags.map(tag => <span key={tag.name} className={`tag tag-${tag.color}`}>{tag.name}</span>)}
      {points ? `+${points} points` : ''}
    </div>
  </>
}