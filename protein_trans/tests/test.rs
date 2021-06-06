#![allow(warnings)]
extern crate protein_trans as prott;

#[cfg(test)]
mod tests
{
    #[test]
    fn emptyRNA()
    {
        let rna = "";
        let result = prott::decode(rna);
        assert!(result.is_err());

    }

    #[test]
    fn badLength()
    {
        let rna = "UA";
        let result = prott::decode(rna);
        assert!(result.is_err());
    }


    #[test]
    fn badProtein()
    {
        let rna = "CCC";
        let result = prott::decode(rna);
        assert!(result.is_err());

    }

    #[test]
    fn stopTranslationMiddle()
    {
        let rna = "AUGUUUUCUUAAAUG";
        let result = prott::decode(rna);
        assert_eq!(result.unwrap(), vec!["Methionine","Phenylalanine","Serine"]);
    }

    #[test]
    fn stopTranslationFirst()
    {
        let rna = "UGAUUUUCUUAAAUG";
        let result = prott::decode(rna);
        assert!(result.unwrap().is_empty());
    }

    #[test]
    fn goodTranslationMultiple()
    {
        let rna = "AUGUUUUCUUAAAUG";
        let result = prott::decode(rna);
        assert_eq!(result.unwrap(),vec!["Methionine","Phenylalanine","Serine"]);
    }

    #[test]
    fn goodTranslationSingle()
    {
        {
            let rna = "UCC";
            let result = prott::decode(rna);
            assert_eq!(result.unwrap(), vec!["Serine"]);
        }
    }



}