using System.ComponentModel.DataAnnotations;
using System.ComponentModel.DataAnnotations.Schema;
using System.Text.Json.Serialization;
using Spork.Server.Data;
using Spork.Server.Persistence.Enums;

namespace Spork.Server.Persistence;

[Table("member_terms")]
public class ParliamentMemberTerm
{
    [Key]
    [Column("id")]
    public Guid Id { get; set; }
    
    public ParliamentMember Member { get; set; }

    [NotMapped]
    public bool IsPartyList => Electorate == null;
    public ParliamentElectorate? Electorate { get; set; }
    
    [Column("date_started")]
    [JsonConverter(typeof(DateOnlyConverter))]
    public DateOnly Started { get; set; }
    
    [Column("start_reason")]
    public TermStartedReason StartReason { get; set; }
    
    [Column("date_ended")]
    [JsonConverter(typeof(DateOnlyConverter))]
    public DateOnly Ended { get; set; }
    
    [Column("end_reason")]
    public TermEndedReason EndReason { get; set; }
}