using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;

namespace Spork.Server.Persistence;

[Table("members")]
public class ParliamentMember
{
    [Key]
    [Column("id")]
    public Guid Id { get; set; }
    
    public ICollection<ParliamentMemberTerm> Terms { get; set; }
}